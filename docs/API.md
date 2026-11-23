# API Contracts (React ↔ Rust)

This document defines the Tauri commands and events used by the React frontend to communicate with the Rust backend.

The Rust backend owns persistent data, domain logic, filesystem access, and platform integrations. The frontend must not access SQLite or implement domain rules independently.

## 1. General Conventions

### 1.1 Serialization

- All JSON keys use `camelCase`.
- Rust structures use `#[serde(rename_all = "camelCase")]`.
- Timestamps use ISO-8601 UTC strings.
- IDs are opaque strings.
- Durations are seconds unless a field explicitly ends in `_ms`.
- Rust `Option<T>` values map to nullable TypeScript values.
- Engine-specific types must not cross the IPC boundary.

### 1.2 Error format

Commands return a consistent error structure:

```ts
type AppError = {
  code:
    | "not_found"
    | "invalid_input"
    | "conflict"
    | "unavailable_offline"
    | "engine_error"
    | "permission_denied"
    | "io_error"
    | "locked"
    | "unsupported"
    | "internal";
  message: string;
  details: unknown | null;
};
```

Example:

```json
{
  "code": "not_found",
  "message": "Media item not found",
  "details": null
}
```

Error messages must be safe to display. They must not expose stack traces, database internals, passcodes, or sensitive filesystem information.

### 1.3 Validation

- Frontend TypeScript validates input shape for immediate feedback.
- Backend Serde validates deserialized input.
- Backend domain logic performs authoritative validation.
- The backend validates enum values, ownership, references, allowed media types, numeric ranges, file formats, and state transitions.
- The frontend must treat backend responses as untrusted runtime data and handle failures.

### 1.4 Command behaviour

- Commands are asynchronous where filesystem, database, metadata, or media-engine work may take time.
- Mutations must use database transactions where multiple related records are changed.
- Successful mutations affecting library data emit `library_changed`.
- Commands must not silently create, merge, or link library entries based on fuzzy matching.
- Original user media files must never be copied, moved, renamed, modified, or deleted.

---

## 2. Profile and Application Security

### `get_profile()` → `Profile`

```ts
type Profile = {
  id: string;
  name: string;
  pictureUri: string | null;
};
```

Returns the single V1 profile.

### `update_profile(input: UpdateProfileInput)` → `Profile`

```ts
type UpdateProfileInput = {
  name?: string;
  pictureUri?: string | null;
};
```

Updates editable profile fields. The backend validates the name and picture reference.

### `get_security_status()` → `SecurityStatus`

```ts
type SecurityStatus = {
  passcodeEnabled: boolean;
  lockedUntil: string | null;
};
```

Returns whether a passcode is configured and whether a temporary lockout is active. Must not return the passcode hash.

### `set_passcode(input: SetPasscodeInput)` → `{ ok: true }`

```ts
type SetPasscodeInput = {
  currentPasscode?: string;
  newPasscode: string | null;
};
```

Sets, changes, or removes the passcode.

- `newPasscode: null` removes the passcode.
- Setting a passcode for the first time does not require `currentPasscode`.
- Changing or removing an existing passcode requires the current passcode.
- The backend stores only a salted password hash using a modern password KDF, preferably Argon2id.
- The passcode does not encrypt the database.

### `verify_passcode(input: { passcode: string })` → `PasscodeVerification`

```ts
type PasscodeVerification = {
  ok: boolean;
  lockedUntil: string | null;
};
```

Verifies the passcode and updates failed-attempt and lockout state as appropriate.

The backend enforces lockout rules. The frontend must not implement its own authoritative attempt counter.

---

## 3. Library

### `list_media_items(query?: ListQuery)` → `MediaItemList`

```ts
type ListQuery = {
  mediaType?: "movie" | "tv_show" | "book";
  search?: string;
  sort?: "title" | "added" | "recent";
  offset?: number;
  limit?: number;
};

type MediaItemSummary = {
  id: string;
  mediaType: "movie" | "tv_show" | "book";
  title: string;
  posterUri: string | null;
  releaseYear: number | null;
  hasLocalFiles: boolean;
};

type MediaItemList = {
  items: MediaItemSummary[];
  total: number;
};
```

Returns paginated library entries. `hasLocalFiles` indicates whether the entry has an available associated resource.

### `get_media_item(input: { id: string })` → `MediaItemDetail`

Returns type-specific detail as a discriminated union on `mediaType`.

The detail includes applicable metadata, external identifiers, tracking data, local resource links, and book annotation information.

### `create_media_item(input: CreateMediaItemInput)` → `MediaItemDetail`

```ts
type SeasonInput = {
  seasonNumber: number;
  title?: string;
  episodes?: EpisodeInput[];
};

type EpisodeInput = {
  episodeNumber: number;
  title: string;
  description?: string;
  runtimeSeconds?: number;
};

type EditionInput = {
  title?: string;
  publisher?: string;
  publicationYear?: number;
  isbn13?: string;
  format: "physical" | "ebook" | "pdf" | "other";
};

type CreateMediaItemInput =
  | {
      mediaType: "movie";
      title: string;
      description?: string;
      releaseYear?: number;
      runtimeSeconds?: number;
    }
  | {
      mediaType: "tv_show";
      title: string;
      description?: string;
      releaseYear?: number;
      seasons?: SeasonInput[];
    }
  | {
      mediaType: "book";
      title: string;
      description?: string;
      firstPublishedYear?: number;
      editions?: EditionInput[];
    };
```

Creates a library entry. Local files and online metadata are not required.

### `update_media_item(input: UpdateMediaItemInput)` → `MediaItemDetail`

Updates editable fields on an existing entry.

The update is partial. Type-specific fields are accepted only for the matching media type. The backend rejects attempts to change an entry's media type through this command.

### `delete_media_item(input: { id: string })` → `{ ok: true }`

Deletes a library entry and applies the database's defined relationship and cascade rules.

It must not delete original files. Local resource records must not be deleted merely because a library entry is removed.

---

## 4. TV Seasons and Episodes

### `list_seasons(input: { tvShowId: string })` → `{ seasons: Season[] }`

Returns seasons belonging to a TV show.

### `list_episodes(input: { seasonId: string })` → `{ episodes: Episode[] }`

Returns episodes belonging to a season.

### `upsert_season(input: SeasonInputWithId)` → `Season`

Creates or updates a season.

### `upsert_episode(input: EpisodeInputWithId)` → `Episode`

Creates or updates an episode.

### `delete_season(input: { id: string })` → `{ ok: true }`

Deletes a season according to the database cascade rules.

### `delete_episode(input: { id: string })` → `{ ok: true }`

Deletes an episode according to the database cascade rules.

The backend validates season and episode relationships and prevents duplicate numbering within the same parent.

---

## 5. Tracking

Tracking commands accept partial updates. Omitted fields remain unchanged; explicit `null` clears nullable fields.

### `update_movie_tracking(input: MovieTrackingPatch)` → `MovieTracking`

```ts
type MovieTrackingPatch = {
  mediaItemId: string;
  status?: "want_to_watch" | "watching" | "completed";
  playbackSeconds?: number;
  durationSeconds?: number | null;
  rating10?: number | null;
};
```

Validates nonnegative playback and duration values and ratings in the 1–10 range.

### `update_show_tracking(input: ShowTrackingPatch)` → `ShowTracking`

Updates show-level tracking, including its optional rating.

Show progress is derived from episode tracking. It must not become an independent, conflicting progress value.

### `update_episode_tracking(input: EpisodeTrackingPatch)` → `EpisodeTracking`

```ts
type EpisodeTrackingPatch = {
  episodeId: string;
  watched?: boolean;
  playbackSeconds?: number;
  durationSeconds?: number | null;
};
```

When an episode is marked watched, the backend records `watchedAt`. When marked unwatched, the watched timestamp is cleared.

Playback progress updates must not silently mark an episode watched.

### `update_book_tracking(input: BookTrackingPatch)` → `BookTracking`

```ts
type BookTrackingPatch = {
  mediaItemId: string;
  status?: "want_to_read" | "reading" | "completed";
  currentPage?: number | null;
  progressPercent?: number | null;
  ratingStars?: number | null;
  formatKind?: "physical" | "digital" | null;
  preferredEditionId?: string | null;
};
```

Validates nonnegative page progress, percentage between 0 and 100, rating between 1 and 5, and edition ownership.

Physical reading progress does not require a digital resource.

### `get_show_progress(input: { tvShowId: string })` → `ShowProgress`

```ts
type ShowProgress = {
  watched: number;
  total: number;
  nextEpisodeId: string | null;
};
```

Returns the number of watched episodes, total episodes, and next unwatched episode according to the product's episode ordering rules.

---

## 6. Crates

### `list_crates()` → `{ crates: Crate[] }`

```ts
type Crate = {
  id: string;
  name: string;
  description: string | null;
  allowedMediaTypes: ("movie" | "tv_show" | "book")[];
  itemCount: number;
};
```

### `create_crate(input: CreateCrateInput)` → `Crate`

```ts
type CreateCrateInput = {
  name: string;
  description?: string;
  allowedMediaTypes: ("movie" | "tv_show" | "book")[];
};
```

### `update_crate(input: CratePatch)` → `Crate`

Updates crate fields. The backend validates changes to allowed media types against existing memberships.

### `delete_crate(input: { id: string })` → `{ ok: true }`

Deletes the crate and its memberships, not the library entries.

### `list_crate_items(input: { crateId: string })` → `{ items: MediaItemSummary[] }`

### `add_to_crate(input: { crateId: string; mediaItemId: string })` → `{ ok: true }`

Fails with `invalid_input` if the entry's media type is not allowed.

### `remove_from_crate(input: { crateId: string; mediaItemId: string })` → `{ ok: true }`

---

## 7. Local Resources and Watched Folders

### `pick_files(input: { purpose: "media" | "subtitle" })` → `{ resources: LocalResource[] }`

Opens the platform file picker and registers selected files as local resources.

Linux and Android implementations use their respective platform file-access mechanisms.

### `pick_folder()` → `{ folder: WatchedFolder }`

Opens the platform folder picker and registers the selected folder for discovery.

### `list_watched_folders()` → `{ folders: WatchedFolder[] }`

### `update_watched_folder(input: WatchedFolderPatch)` → `WatchedFolder`

Updates watched-folder settings.

### `delete_watched_folder(input: { id: string })` → `{ ok: true }`

Removes the watched-folder registration and its discovery mappings. Does not delete files or library entries.

### `scan_folder(input: { folderId: string })` → `{ scanId: string }`

Starts an asynchronous scan and emits `scan_progress` and `scan_complete`.

### `list_scan_results(input: { scanId: string })` → `{ files: ScanFileResult[] }`

```ts
type ScanFileResult = {
  resourceId: string;
  displayName: string;
  candidates: MatchCandidate[];
};

type MatchCandidate = {
  mediaItemId: string;
  title: string;
  confidence: number;
  kind: "existing_link" | "suggested";
};
```

`confidence` is informational only. It must never cause automatic matching.

### `confirm_resource_match(input: ConfirmResourceMatchInput)` → `{ ok: true }`

```ts
type ConfirmResourceMatchInput = {
  mediaItemId: string;
  resourceId: string;
  kind: "primary" | "additional" | "subtitle";
  subtitleLanguage?: string;
};
```

Creates a user-confirmed resource link. The backend validates the resource, library entry, and link kind.

### `reject_resource_match(input: { resourceId: string; mediaItemId: string })` → `{ ok: true }`

Rejects a suggestion for the current scan session. Rejection is not persisted.

### `create_media_item_from_resource(input: CreateFromResourceInput)` → `MediaItemDetail`

```ts
type CreateFromResourceInput = {
  resourceId: string;
  mediaType: "movie" | "tv_show" | "book";
  title: string;
  kind: "primary" | "additional";
};
```

Creates a new library entry and links the selected resource.

### `unlink_resource(input: { mediaItemId: string; resourceId: string })` → `{ ok: true }`

Removes the association between a library entry and a resource. Does not delete either the resource record or the original file.

### `delete_resource_reference(input: { resourceId: string })` → `{ ok: true }`

Explicitly deletes a local resource reference and its applicable associations. Never deletes the original file.

---

## 8. Video Playback

### `playback_open(input: PlaybackOpenInput)` → `{ sessionId: string }`

```ts
type PlaybackOpenInput = {
  mediaItemId?: string;
  episodeId?: string;
  resourceId?: string;
};
```

Resolves the requested library entry or episode to an available video resource and starts a playback session.

The backend validates that the selected resource is playable and belongs to the requested entry when both are supplied.

### `playback_control(input: PlaybackControlInput)` → `{ ok: true }`

```ts
type PlaybackControlInput = {
  sessionId: string;
  action:
    | "play"
    | "pause"
    | "seek"
    | "set_speed"
    | "set_volume"
    | "select_audio"
    | "select_subtitle"
    | "attach_subtitle"
    | "set_subtitle_delay"
    | "set_audio_delay"
    | "enter_fullscreen"
    | "exit_fullscreen";
  value?: number | string | null;
};
```

The meaning and expected value type depend on the action. The backend validates the action and value before passing the request to the platform media engine.

### `playback_close(input: { sessionId: string })` → `{ ok: true }`

Stops the session and persists final playback position and applicable duration to tracking.

### `playback_get_state(input: { sessionId: string })` → `PlaybackState`

```ts
type PlaybackState = {
  positionSeconds: number;
  durationSeconds: number | null;
  playing: boolean;
  speed: number;
  volume: number;
  audioTracks: MediaTrack[];
  subtitleTracks: MediaTrack[];
  error: AppError | null;
};

type MediaTrack = {
  id: string;
  label: string;
  language?: string;
};
```

Returns the current session state.

Engine-specific track identifiers are opaque and valid only for the current session.

---

## 9. Book Reader

### `reader_open(input: ReaderOpenInput)` → `ReaderSession`

```ts
type ReaderOpenInput = {
  mediaItemId: string;
  resourceId: string;
};

type ReaderSession = {
  sessionId: string;
  format: "epub" | "pdf";
  fileUrl: string;
};
```

Opens an EPUB or PDF resource and returns a session-scoped URL for the reader.

`fileUrl` is valid only for the active session. It must use the application's controlled resource-access mechanism and must not expose unrestricted filesystem access.

### `reader_update_position(input: ReaderPositionPatch)` → `{ ok: true }`

```ts
type ReaderPositionPatch = {
  sessionId: string;
  position: string;
  currentPage?: number | null;
  progressPercent?: number | null;
};
```

Persists the current reading position and applicable book progress. The backend validates the session and values.

### `reader_get_annotations(input: { sessionId: string })` → `{ annotations: Annotation[] }`

Returns annotations applicable to the active book resource and edition.

### `reader_begin_selection(input: { sessionId: string })` → `{ selection: TextSelection }`

```ts
type TextSelection = {
  format: "epub" | "pdf";
  selectorJson: string;
  quotedText: string;
};
```

Returns the current text selection and its location selector. The selector structure follows `docs/DATABASE.md`.

### `reader_close(input: { sessionId: string })` → `{ ok: true }`

Closes the reader session and persists the final reading position.

---

## 10. Annotations

### `annotation_create(input: AnnotationInput)` → `Annotation`

```ts
type AnnotationInput = {
  mediaItemId: string;
  bookEditionId?: string;
  resourceId?: string;
  kind: "highlight" | "note";
  format: "epub" | "pdf";
  selectorJson: string;
  quotedText: string;
  note?: string;
  color?: string;
};
```

Creates an annotation.

- A highlight may contain an attached note.
- A standalone note uses `kind: "note"`.
- The backend validates the book, edition, resource, format, and selector.
- An annotation is stored in Colkt's database and never written into the original book file.

### `annotation_update(input: AnnotationUpdateInput)` → `Annotation`

```ts
type AnnotationUpdateInput = {
  id: string;
  note?: string | null;
  color?: string | null;
};
```

Updates an annotation's note or color. `null` clears a nullable field.

### `annotation_delete(input: { id: string })` → `{ ok: true }`

Deletes the annotation. Deleting a highlight also removes its attached note because the note belongs to that highlight.

### `annotation_list(input: { mediaItemId: string })` → `{ annotations: Annotation[] }`

Returns annotations associated with the library entry.

Annotation selectors must be treated as potentially stale. The reader must not silently attach an unresolved annotation to unrelated content.

---

## 11. Search and Metadata

### `search_local(input: LocalSearchInput)` → `{ items: MediaItemSummary[] }`

```ts
type LocalSearchInput = {
  text: string;
  mediaType?: "movie" | "tv_show" | "book";
};
```

Searches the local library. Must work offline.

### `search_online(input: OnlineSearchInput)` → `{ results: ExternalSearchResult[] }`

```ts
type OnlineSearchInput = {
  provider: string;
  query: string;
  mediaType: "movie" | "tv_show" | "book";
};
```

Searches a supported metadata provider.

Provider identifiers and supported media types are validated by the backend. Network errors and provider limitations must be reported without affecting local library access.

### `apply_metadata(input: ApplyMetadataInput)` → `MediaItemDetail`

```ts
type ApplyMetadataInput = {
  mediaItemId: string;
  provider: string;
  externalId: string;
  fields: Partial<MediaItemMetadata>;
};
```

Applies user-approved metadata to an existing entry and records the external identifier.

The backend validates provider data and permitted fields. It must not merge entries automatically.

---

## 12. Backup and Restore

### `backup_export(input: BackupExportInput)` → `{ path: string }`

```ts
type BackupExportInput = {
  destinationPath: string;
  includeMetadataCache?: boolean;
};
```

Exports application data into a versioned backup.

- Original media files are excluded.
- Local resource references are included.
- SQLite data must be captured consistently, accounting for WAL and active transactions.
- Export must not interrupt normal library use unnecessarily.

### `backup_preview(input: { sourcePath: string })` → `BackupPreview`

```ts
type BackupPreview = {
  schemaVersion: string;
  createdAt: string;
  counts: {
    mediaItems: number;
    crates: number;
    annotations: number;
  };
  compatible: boolean;
};
```

Validates and previews a backup without restoring it.

### `backup_restore(input: BackupRestoreInput)` → `BackupRestoreResult`

```ts
type BackupRestoreInput = {
  sourcePath: string;
  mode: "replace" | "merge";
};

type BackupRestoreResult = {
  ok: true;
  warnings: string[];
};
```

Restores application data.

- Replace mode requires explicit user confirmation in the frontend.
- Restore must be transactional.
- Incompatible schema versions are rejected.
- Missing local resources are retained as references and marked unavailable.
- A failed restore must not leave partially restored data.

---

## 13. Settings

### `get_setting(input: { key: string })` → `{ value: unknown } | null`

Returns a setting or `null` when it does not exist.

### `set_setting(input: { key: string; value: unknown })` → `{ ok: true }`

Stores a setting. The backend validates recognized setting keys and their values.

### `list_settings()` → `{ entries: SettingEntry[] }`

```ts
type SettingEntry = {
  key: string;
  value: unknown;
};
```

Returns application settings. Sensitive values such as passcode hashes must never be exposed through this interface.

---

## 14. Events

Events are emitted by the Rust backend and consumed by the frontend through Tauri's event API.

### `library_changed`

Fired after a successful mutation affecting library data.

```ts
type LibraryChangedPayload = {
  reason: string;
};
```

The frontend should refresh affected library views.

### `scan_progress`

```ts
type ScanProgressPayload = {
  scanId: string;
  processed: number;
  total?: number;
  currentName?: string;
};
```

Reports asynchronous folder-scan progress.

### `scan_complete`

```ts
type ScanCompletePayload = {
  scanId: string;
  files: number;
  candidates: number;
  errors: string[];
};
```

Signals completion of a folder scan.

### `playback_state`

```ts
type PlaybackStatePayload = {
  sessionId: string;
  state: PlaybackState;
};
```

Emitted when playback opens and on meaningful state changes, including play, pause, seek, track changes, and playback end.

### `playback_error`

```ts
type PlaybackErrorPayload = {
  sessionId: string;
  code: string;
  message: string;
};
```

Reports a playback-session error.

### `reader_error`

```ts
type ReaderErrorPayload = {
  sessionId: string;
  code: string;
  message: string;
};
```

Reports a reader-session error.

### `backup_progress`

```ts
type BackupProgressPayload = {
  operation: "export" | "restore";
  processed: number;
  total?: number;
};
```

Reports backup or restore progress.

### `app_error`

Carries an `AppError` for background failures that are not associated with a specific command response.

```ts
type AppErrorPayload = AppError;
```

---

## 15. Frontend IPC Rules

1. All persistent data access goes through documented Tauri commands.
2. The frontend must not access SQLite directly.
3. React components should call feature-level frontend services or hooks rather than scattering raw IPC calls throughout the UI.
4. Frontend services must handle command errors and expose typed results.
5. Event listeners must be removed when their owning component or service is disposed.
6. Long-running operations must expose progress and failure states.
7. Session-bound commands must use the session ID returned by the corresponding open command.
8. A closed or invalid session must return an appropriate error.
9. The frontend must not assume that a command succeeded until its promise resolves successfully.
10. Raw filesystem paths and platform-specific resource identifiers must not be exposed to general UI components. Use controlled resource references and display names instead.

## 16. Contract Maintenance

- This document is the source of truth for frontend-to-backend command and event contracts.
- Any command or event change must update this document and its corresponding Rust and TypeScript types.
- Database changes must also update `docs/DATABASE.md` and use a new versioned migration.
- Feature changes must remain consistent with `docs/FEATURES.md`.
- Platform-specific behaviour must remain consistent with `docs/PLATFORM.md` and `docs/MEDIA.md`.
- Do not add undocumented commands, event payloads, or frontend-only substitutes for backend domain operations.