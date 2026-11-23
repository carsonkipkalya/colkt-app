# Features (V1)

Legend: **[Required]**, **[Optional]**, **[Deferred]**.

## A. Application Foundation

### A1. Single local profile **[Required]**

The app creates one profile on first launch. The profile name and picture are editable. Multiple profiles are outside the scope of V1.

**Acceptance criteria**
- First launch creates exactly one default profile.
- Profile name can be edited and persisted.
- Picture can be selected from a local file and displayed.
- Profile data remains available offline.
- No concept of multiple profiles in V1.

### A2. App-level passcode **[Required]**

An optional passcode gates access to the application. It is an access-control feature, not database encryption.

**Acceptance criteria**
- User can set, change, and remove a passcode.
- When enabled, the app displays its lock screen before granting access to the library.
- Passcode is requested on cold launch.
- On Android, passcode is requested when returning from the background, according to the app's locking behaviour.
- Wrong passcode increments the failure counter; too many failures cause a temporary lockout.
- Passcode is stored only as a salted password hash using a modern KDF, preferably Argon2id.
- Passcode does not encrypt the database. The UI must state this clearly.

### A3. Adaptive layouts **[Required]**

Desktop and mobile layouts adapt to available width and platform.

**Acceptance criteria**
- Navigation is usable at 360 px and 1440 px widths.
- Desktop and mobile layouts remain usable without requiring identical navigation patterns.
- Player and reader have dedicated full-screen modes.

## B. Library

### B1. Unified library **[Required]**

Movies, TV shows, and books appear in one library.

**Acceptance criteria**
- Library list supports filtering by media type.
- Sorting supports title, date added, and recently accessed or updated items.
- Offline search covers titles, sort titles, and descriptions.
- Empty state is shown when the library contains no items.
- Library entries can exist without associated local files.

### B2. Manual item creation **[Required]**

Users can create library entries without files or metadata from an online provider.

**Acceptance criteria**
- Create movie entries.
- Create TV show entries, including seasons and episodes.
- Create book work entries with editions.
- Required fields are media type and title.
- User can edit all editable fields afterwards.
- Creating an entry does not require selecting a local file.

### B3. Metadata retrieval **[Required]**

Retrieve metadata and artwork from configured providers when a network connection is available.

**Acceptance criteria**
- Provider integrations are isolated behind backend adapters.
- Retrieved metadata is cached locally.
- User can review suggested metadata before applying it.
- User can override editable metadata fields manually.
- Network failures produce recoverable errors.
- Manual creation and editing remain available offline.
- Metadata matching never automatically merges library entries.

### B4. Media type filter and multi-select **[Optional]**

Support selecting multiple library entries for bulk actions.

**Acceptance criteria**
- Users can select multiple entries.
- Bulk add to crate and bulk status changes may be supported.
- This feature is not required for the first usable version.

## C. Crates

### C1. User-defined collections **[Required]**

Users can create collections called crates to organise library entries.

**Acceptance criteria**
- Create, rename, and delete crates.
- Each crate has an allowed media-type configuration: any subset of movie, TV show, and book.
- Add and remove library entries.
- Crate membership is independent of local files and tracking.
- Entries of a disallowed media type cannot be added.
- Deleting a crate does not delete its library entries or their tracking data.

### C2. Crate ordering **[Optional]**

Support manual ordering of entries within a crate.

## D. Tracking

### D1. Movie tracking **[Required]**

Track movie status, playback progress, and ratings.

**Acceptance criteria**
- Status supports Want to Watch, Watching, and Completed.
- Status can be changed freely.
- Playback position is persisted periodically and when playback stops or closes.
- User can reset playback progress.
- Rating is optional, uses a 1–10 scale, and can be removed.
- Continue Watching surfaces relevant in-progress movies.

### D2. TV show tracking **[Required]**

Track show, season, and episode progress, including watched history.

**Acceptance criteria**
- Mark individual episodes watched or unwatched.
- Record `watched_at` when an episode is marked watched.
- Preserve watched history when playback progress changes.
- Next Episode is the lowest-numbered unwatched episode across the show's seasons.
- Overall progress is calculated as watched episodes divided by total episodes.
- Show rating is optional, uses a 1–10 scale, and can be removed.
- Show-level progress is derived from episode tracking rather than maintained as an unrelated independent value.

### D3. Book tracking **[Required]**

Track reading status, progress, format classification, and ratings.

**Acceptance criteria**
- Status supports Want to Read, Reading, and Completed.
- Progress can be entered as a page number or percentage.
- Page and percentage progress are stored where applicable.
- Format classification distinguishes physical and digital reading.
- Format classification is optional but persisted when supplied.
- Rating is optional, uses a 1–5 scale, and can be removed.
- Physical reading progress is supported without requiring an associated digital file.

## E. Local Media

### E1. File and folder selection **[Required]**

Users can grant Colkt access to individual files or folders.

**Acceptance criteria**
- Linux uses an appropriate native file or folder picker through the portal or Tauri dialog.
- Android uses Storage Access Framework (SAF) file and tree pickers.
- Selected files become local resource records.
- Folder selections can be registered for discovery and scanning.
- Original files are never copied, moved, renamed, modified, or deleted by the app.
- Access permissions and limitations are handled according to the platform.

### E2. Recursive folder scan **[Required]**

Scan selected folders recursively and produce discovery results.

**Acceptance criteria**
- Scanning runs asynchronously.
- Scan progress is reported to the frontend through backend events.
- Results are grouped by candidate media.
- Re-running a scan does not create duplicate resource records.
- Unsupported, inaccessible, and unreadable files are handled without terminating the entire scan.
- Scanning does not automatically create or merge library entries.

### E3. Folder monitoring **[Optional on Linux, Deferred on Android]**

Monitor watched folders for changes where the platform permits it.

**Acceptance criteria**
- Linux folder monitoring is optional and subject to sandbox permissions.
- Android folder monitoring is deferred.
- The app must continue to work when monitoring is unavailable.

### E4. Scan on startup **[Required]**

Allow watched folders to be scanned automatically when the app launches.

**Acceptance criteria**
- Setting to enable or disable startup scanning.
- Scanning runs in the background.
- Failures are reported non-fatally.
- Startup scanning does not block access to the rest of the library.

### E5. Remove local file reference **[Required]**

Users can unlink a file from a library entry or remove its local resource reference without deleting the original file.

**Acceptance criteria**
- Removing a media-resource link does not delete the resource record or library entry.
- Removing a resource reference never deletes the original file.
- A missing or inaccessible file can be marked unavailable.
- A resource record is deleted only when the user explicitly chooses to delete that reference.
- Removing a file reference does not remove tracking, ratings, annotations, or crate membership.

## F. Media Matching

### F1. Candidate suggestions **[Required]**

Suggest possible matches between discovered files and existing library entries.

**Acceptance criteria**
- Candidates are derived from filename parsing and fuzzy matching.
- Multiple candidates are shown when a match is ambiguous.
- Suggestions do not automatically create links or merge entries.
- Unmatched files remain available for user review.

### F2. User confirmation **[Required]**

Users decide whether discovered files belong to existing entries or should create new entries.

**Acceptance criteria**
- User can confirm or reject a suggested match.
- Confirming a match creates a media-resource link.
- Rejecting a candidate hides that suggestion for the current session.
- User can create a new library entry from a discovered file.
- Creating a new entry from a file links the new entry to that resource.
- Linking a file to an existing entry preserves its tracking, ratings, annotations, and crate membership.
- No fuzzy match is ever auto-merged.

### F3. Multiple files per entry **[Required]**

A library entry can have multiple associated local resources.

**Acceptance criteria**
- Multiple resource links are allowed for one entry.
- Link kind distinguishes primary, additional, and subtitle resources.
- Users can choose among associated video resources for playback.
- Removing one link does not remove other links or the library entry.

## G. Video Playback

### G1. In-app playback **[Required, provisional on both platforms]**

Play associated local video files inside Colkt.

**Acceptance criteria**
- Play and pause.
- Seek.
- Volume control.
- Playback speed control.
- Full-screen mode.
- Resume from the saved playback position.
- Playback progress is reported to the backend periodically and on stop.
- Multiple audio and subtitle tracks are available where supported by the selected engine.
- External subtitle attachment is supported where the engine permits it.
- Audio/video or subtitle synchronisation offset is supported where the engine permits it.
- Playback engine limitations are documented in `docs/MEDIA.md`.

Format support depends on the selected engine, operating system, and available codecs. Colkt must not promise universal format support.

## H. Book Reader

### H1. EPUB and PDF reading **[Required, provisional]**

Open and read local EPUB and PDF files inside Colkt.

**Acceptance criteria**
- EPUB reading uses Foliate.js provisionally.
- PDF reading uses Mozilla PDF.js provisionally.
- Reading position is persisted.
- Page or percentage progress is updated and reflected in book tracking.
- Reader implementations remain behind the reader-engine interface.
- Original book files are never modified.

### H2. Highlights and annotations **[Required]**

Allow users to highlight text and attach notes while reading.

**Acceptance criteria**
- Text selection can create a highlight with a stable anchor.
- Notes can be attached to highlights.
- Attached notes can be edited and deleted.
- Deleting a highlight also deletes its attached note.
- Annotations are stored in the database, not written into the original book file.
- Annotations are reloaded and rendered when the book is reopened.
- If an annotation anchor can no longer be resolved, the app must not silently attach it to unrelated text.

## I. Search

### I1. Offline library search **[Required]**

Search the local library without a network connection.

**Acceptance criteria**
- Search covers titles and descriptions.
- Results update as the user types, with debouncing.
- Search works fully offline.
- Search results respect the library's media types and available filters.

### I2. Online catalogue search **[Required]**

Search supported metadata providers when connected.

**Acceptance criteria**
- Provider integrations use backend adapters.
- Results can be applied to an existing library entry or used to create a new one.
- User reviews metadata before applying it.
- Network and provider errors are surfaced clearly.
- Offline mode falls back to local search and manual entry.
- Online search does not automatically merge library entries.

## J. Backup and Restore

### J1. Export **[Required]**

Export Colkt application data into a versioned backup file.

**Acceptance criteria**
- Backup includes profile, library entries, tracking, crates, annotations, settings, watched folders, local resource references, external identifiers, and schema version.
- Metadata cache may be included optionally.
- Original media files are excluded by default.
- Backup is produced as a single file, such as a ZIP container.
- Database export accounts for SQLite transaction and WAL consistency.
- Backup format is versioned so future versions can identify compatibility.

### J2. Restore **[Required]**

Restore application data from a backup.

**Acceptance criteria**
- Incompatible schema versions are rejected with a clear message.
- Restore supports merge and replace modes.
- Replace mode requires explicit confirmation.
- Missing or inaccessible local resources are retained as references and marked unavailable, not deleted from restored data.
- Restore is transactional.
- A failed restore must not leave the library partially restored.

## K. Deferred for V1

The following are outside the scope of V1:

- Multiple profiles.
- Cloud sync.
- Online accounts.
- Streaming from remote sources.
- OCR or full-text search inside books.
- Social features and sharing.
- External recommendations.
- Automatic metadata refresh policies beyond cached results.