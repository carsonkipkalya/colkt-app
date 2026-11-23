# Media Engine and Reader Engine Integration

Colkt uses separate abstractions for video playback and book reading. The core domain must not depend on engine-specific types or APIs.

Video engines are native platform integrations. EPUB and PDF rendering use frontend web-based engines, with Rust responsible for resource access, session management, validation, and persistence.

All engine selections remain provisional until validated through working prototypes.

## 1. Video Playback

### 1.1 Interface

The shared Rust interface isolates the application from platform-specific playback implementations.

```rust
pub trait MediaEngine: Send + Sync {
    fn open(&self, resource: ResourceHandle) -> Result<EngineSession, MediaError>;

    fn control(
        &self,
        session: &EngineSession,
        action: ControlAction,
    ) -> Result<(), MediaError>;

    fn state(&self, session: &EngineSession) -> Result<PlaybackState, MediaError>;

    fn close(&self, session: EngineSession) -> Result<(), MediaError>;
}
```

```rust
pub enum ControlAction {
    Play,
    Pause,
    Seek(f64),
    SetSpeed(f64),
    SetVolume(f64),
    SelectAudioTrack(String),
    SelectSubtitleTrack(Option<String>),
    AttachSubtitle(ResourceHandle),
    SetSubtitleDelay(f64),
    SetAudioDelay(f64),
    EnterFullscreen,
    ExitFullscreen,
}
```

The exact internal types may evolve during implementation, but the shared interface must preserve these capabilities.

`ResourceHandle` is a backend-only wrapper around either a filesystem resource or an Android SAF content URI. It must never be exposed directly to the frontend.

`EngineSession` is an opaque backend-owned session identifier. The frontend refers to playback sessions using the API's `sessionId`.

### 1.2 Provisional Implementations

<badge color="warning">Provisional</badge>

- **Linux:** `GStreamerEngine`, using `gstreamer-rs`.
- **Android:** `Media3Engine`, integrating AndroidX Media3 through the Android/Tauri native integration layer.

Neither implementation is considered settled until a prototype validates the requirements below.

### 1.3 Prototype Requirements

Validate each engine against:

- Opening local resources.
- Rendering video inside the Colkt application.
- Play and pause.
- Seeking.
- Playback speed control.
- Volume control.
- Audio-track enumeration and selection.
- Subtitle-track enumeration and selection.
- External subtitle attachment.
- Subtitle delay.
- Audio delay.
- Full-screen playback.
- Playback position reporting.
- Duration reporting where available.
- Resource cleanup when playback closes.
- Platform lifecycle behaviour.
- Hardware decoding where available.
- Access to files selected through the platform's supported file-access mechanism.

A platform engine must not be considered complete merely because it can play a video.

If an engine cannot satisfy a required capability, document the limitation and evaluate alternatives before closing the relevant roadmap phase.

### 1.4 Playback State

The shared playback state contains:

```rust
pub struct PlaybackState {
    pub position_seconds: f64,
    pub duration_seconds: Option<f64>,
    pub playing: bool,
    pub speed: f64,
    pub volume: f64,
    pub audio_tracks: Vec<MediaTrack>,
    pub subtitle_tracks: Vec<MediaTrack>,
    pub error: Option<MediaError>,
}

pub struct MediaTrack {
    pub id: String,
    pub label: String,
    pub language: Option<String>,
}
```

Track identifiers are opaque and valid only for the active playback session.

The frontend must not depend on GStreamer or Media3 track identifiers or internal data structures.

### 1.5 State Flow

- The Rust backend owns playback sessions.
- The frontend opens playback through `playback_open`.
- The backend returns a `sessionId`.
- Playback commands are sent through `playback_control`.
- The frontend can retrieve the current state through `playback_get_state`.
- Meaningful playback changes are reported through `playback_state` events.
- Playback failures are reported through `playback_error` events and appropriate command errors.
- The frontend closes playback through `playback_close`.

The backend remains responsible for session lifecycle and persistence.

### 1.6 Progress Synchronisation

Movie progress is stored in:

- `movie_tracking.playback_seconds`
- `movie_tracking.duration_seconds`

Episode progress is stored in:

- `episode_tracking.playback_seconds`
- `episode_tracking.duration_seconds`

Persistence rules:

- Persist progress when playback pauses.
- Persist progress when playback closes.
- Persist progress periodically, no more frequently than once every five seconds.
- Persist duration when the engine reports it.
- Do not rely exclusively on application shutdown callbacks.
- Playback progress updates must not automatically mark an episode watched.

When an episode is explicitly marked watched, the backend sets `watched` and `watched_at`.

Re-watching an episode updates playback progress without clearing its watched history unless the user explicitly marks it unwatched.

### 1.7 Error Handling

- Engine open failures are mapped to the shared `AppError` contract.
- Engine-specific error codes may appear in `details` for diagnostics.
- Expected engine failures must not panic command handlers.
- Corrupt or unsupported media must produce a clear user-facing message.
- An unavailable resource must be reported before attempting playback.
- Failed playback must not delete or unlink the associated resource.
- The player may offer an "Open externally" action only when the platform exposes a safe and supported mechanism.

### 1.8 Engine Replacement

- Implementations remain behind the shared `MediaEngine` interface.
- Platform-specific code lives under the relevant platform integration.
- Domain logic must not depend on the selected engine.
- Engine selection must not alter database models or frontend shared types.
- Changing the selected engine requires an ADR in `docs/DECISIONS.md`.

Do not introduce a generic engine plugin framework unless a concrete requirement justifies it.

---

## 2. Book Reader

### 2.1 Architecture

Colkt uses frontend web-based rendering engines for EPUB and PDF.

- EPUB: Foliate.js, provisional.
- PDF: Mozilla PDF.js.

The React frontend hosts and controls the rendering interface.

Rust owns:

- Reader session creation and lifecycle.
- Access to the original local resource.
- Controlled resource delivery.
- Reading-position persistence.
- Annotation persistence and validation.
- Resource availability checks.

The frontend reader adapter handles rendering and translates engine-specific information into the shared reader command contracts.

### 2.2 Reader Interface

The shared Rust reader interface manages the lifecycle and backend responsibilities of reader sessions.

```rust
pub trait ReaderEngine: Send + Sync {
    fn open(&self, session: ReaderSession) -> Result<(), ReaderError>;

    fn close(&self, session: ReaderSession) -> Result<(), ReaderError>;

    fn get_annotations(
        &self,
        session: ReaderSession,
    ) -> Result<Vec<AnnotationView>, ReaderError>;

    fn validate_selection(
        &self,
        session: ReaderSession,
        selection: TextSelection,
    ) -> Result<TextSelection, ReaderError>;
}
```

The exact division between Rust and frontend reader adapters must remain consistent with the API contracts.

The frontend must not expose Foliate.js or PDF.js internal types to shared application services or Rust domain logic.

### 2.3 Provisional Implementations

<badge color="warning">Provisional</badge>

**EPUB — Foliate.js**

Validate:

- Opening local EPUB files.
- Navigation between chapters.
- Page or position progression.
- Text selection.
- Stable annotation anchors.
- Highlight rendering.
- Attached notes.
- Restoration of reading position.
- Resource loading through the controlled reader URL.

**PDF — Mozilla PDF.js**

Validate:

- Opening local PDF files.
- Page navigation.
- Zoom.
- Text selection.
- Stable page-based annotation anchors.
- Highlight rendering.
- Attached notes.
- Restoration of reading position.
- Resource loading through the controlled reader URL.

Both implementations must operate without modifying the original book file.

### 2.4 Reader Sessions

Opening a book uses:

`reader_open`

The backend validates the library entry, selected resource, and resource availability before creating a session.

The response contains:

```ts
type ReaderSession = {
  sessionId: string;
  format: "epub" | "pdf";
  fileUrl: string;
};
```

`fileUrl` is a session-scoped resource URL.

- It must not expose unrestricted filesystem access.
- It must not be reusable as a general-purpose file URL.
- It must be invalidated when the session closes.
- It must respect platform permissions.
- The original file must never be copied or modified to provide reader access.

The frontend uses `sessionId` for subsequent reader commands.

### 2.5 Reading Progress

Reading progress is stored in `book_tracking`.

Fields:

- `current_page`
- `progress_percent`

Persistence rules:

- Save progress periodically, no more frequently than once per minute.
- Save final progress when `reader_close` is called.
- Do not rely exclusively on application shutdown callbacks.
- Validate page numbers and percentage values in the backend.
- Progress must be restored when the book is reopened.

Format-specific behaviour:

**EPUB**
- Progress percentage is authoritative.
- Page number is optional because pagination can change with font size, screen size, and layout settings.
- Store a stable reading location where the engine supports it.

**PDF**
- Current page is the primary position.
- Progress percentage may be derived from `page / page_count` when page count is available.

Physical book tracking does not require an active reader session or digital resource.

### 2.6 Annotation Selectors

Annotations must use stable, format-aware selectors stored as `selectorJson`.

**EPUB**

Use a CFI-based location descriptor with a quoted-text fallback.

**PDF**

Store:

- Page number.
- Normalised rectangle coordinates in page space.
- Quoted text where available.

The exact selector representation must be versioned or otherwise identifiable so future reader implementations can interpret existing annotations.

Engine-specific objects, such as Foliate.js CFI parser types or PDF.js viewport objects, must be converted at the reader boundary.

### 2.7 Annotation Lifecycle

- Annotations are created only through `annotation_create`.
- The backend validates the selector and associated book, edition, and resource.
- Highlights may contain an attached note.
- Standalone notes are supported.
- Notes and highlights are persisted in SQLite.
- Annotations are returned through `reader_get_annotations` or `annotation_list`.
- The frontend renders annotations using the active reader engine.
- Editing a note or highlight uses `annotation_update`.
- Deleting an annotation uses `annotation_delete`.

If an annotation cannot be resolved in the current document, it must be shown as needing review.

For example, an annotation may become unresolved when a different edition of a book is opened.

The reader must never silently reposition an annotation onto unrelated text.

### 2.8 Reader Errors

- Missing resources are reported before opening.
- Revoked Android SAF permissions are handled as unavailable resources.
- Corrupt or unsupported files produce clear errors.
- Reader errors are mapped to the shared `AppError` contract.
- Background reader failures may be reported through `reader_error`.
- A failed reader session must not delete annotations or tracking data.
- The associated resource remains linked unless the user explicitly removes it.

### 2.9 Reader Engine Replacement

- EPUB and PDF rendering implementations are isolated behind frontend reader adapters.
- Backend session, resource, persistence, and annotation contracts remain stable.
- Changing an engine requires an ADR in `docs/DECISIONS.md`.
- Existing annotations must remain compatible or have an explicit migration strategy.

---

## 3. Resource Access

### 3.1 Resource Handle

`ResourceHandle` is an internal backend abstraction representing a resource that the application has permission to access.

It may wrap:

- A Linux filesystem resource accessed through Flatpak permissions or portals.
- An Android SAF content URI with persisted access permissions.

It must not be serialised into frontend shared types.

### 3.2 Platform Access

**Linux**

- Resolve resources through the platform filesystem adapter.
- Respect Flatpak sandbox restrictions.
- Use portal-backed access where appropriate.
- Report inaccessible resources without assuming unrestricted host filesystem access.

**Android**

- Resolve resources through SAF content URIs.
- Verify persisted URI permissions.
- Do not assume content URIs can be converted to filesystem paths.
- Use Android document APIs for resource operations.

### 3.3 Resource Safety

- Original files are read-only from Colkt's perspective.
- Playback and reading must not modify original files.
- External subtitles are read as separate resources.
- Resource access is limited to the active operation or session where possible.
- Closing a session releases associated handles and temporary resources.
- Revoked permissions or missing files update resource availability without deleting library entries or tracking.

---

## 4. Cross-Engine Rules

1. No engine-specific type may appear in database rows, domain modules, shared frontend types, or Tauri command payloads.
2. Engine handles remain backend-owned.
3. The frontend identifies active operations using opaque session IDs.
4. All persistent state changes go through documented backend commands.
5. Engines must not independently write to SQLite.
6. Original user files must never be copied, moved, renamed, modified, or deleted.
7. Engine failures must not corrupt library entries, tracking, annotations, or resource associations.
8. Platform-specific limitations must be documented.
9. Required capabilities must be validated on the actual target platform.
10. Changing an engine or its architectural integration requires an ADR.

## 5. Validation Checklist

### Linux Video

- [ ] GStreamer prototype opens a selected local video.
- [ ] Video renders inside Colkt.
- [ ] Play, pause, and seek work.
- [ ] Speed and volume controls work.
- [ ] Audio tracks can be selected.
- [ ] Subtitle tracks can be selected.
- [ ] External subtitles can be attached.
- [ ] Audio and subtitle delays work.
- [ ] Playback position and duration are reported.
- [ ] Progress is persisted at the required cadence.
- [ ] Playback works inside the Flatpak sandbox.
- [ ] Sessions close and release resources correctly.

### Android Video

- [ ] Media3 prototype opens a SAF-selected video.
- [ ] Video renders inside Colkt.
- [ ] Play, pause, and seek work.
- [ ] Speed and volume controls work.
- [ ] Audio tracks can be selected.
- [ ] Subtitle tracks can be selected.
- [ ] External subtitles can be attached through SAF.
- [ ] Audio and subtitle delays work.
- [ ] Playback position and duration are reported.
- [ ] Progress survives backgrounding and process recreation.
- [ ] Sessions close and release resources correctly.

### EPUB

- [ ] Foliate.js opens a local EPUB.
- [ ] Chapter navigation works.
- [ ] Reading position is saved and restored.
- [ ] Text selection works.
- [ ] Highlights are created and restored.
- [ ] Notes can be attached, edited, and deleted.
- [ ] Unresolved annotations are shown as needing review.
- [ ] Original EPUB remains unchanged.

### PDF

- [ ] PDF.js opens a local PDF.
- [ ] Page navigation and zoom work.
- [ ] Reading position is saved and restored.
- [ ] Text selection works where supported by the document.
- [ ] Page-based highlights are created and restored.
- [ ] Notes can be attached, edited, and deleted.
- [ ] Unresolved annotations are shown as needing review.
- [ ] Original PDF remains unchanged.

## 6. Known Limitations

- GStreamer and Media3 integrations are provisional.
- Video codec and container support depends on the engine, platform, and installed codecs.
- Hardware decoding availability varies by device.
- Some media formats may not expose all audio or subtitle tracks.
- External subtitle support depends on engine capabilities.
- EPUB pagination may change with reader settings and screen dimensions.
- PDF text selection may be unavailable for scanned or image-only documents.
- Annotation anchors may require review when a different edition or modified document is opened.
- Android resource access depends on SAF provider capabilities and persisted permissions.

These limitations must be reflected in user-facing behaviour rather than hidden behind unsupported feature claims.