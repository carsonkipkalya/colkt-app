# Platform Integration

Colkt targets Linux desktop and Android. Shared functionality is implemented once wherever possible, while platform-specific behaviour is isolated under `src-tauri/src/platform/`.

Platform adapters provide filesystem access, permissions, lifecycle integration, and native capabilities. Shared domain logic must not assume that either platform has capabilities unavailable on the other.

## 1. Shared Behaviour

The following functionality is shared across platforms:

- Domain rules and database access.
- Library, crates, and tracking.
- Metadata caching and provider integration.
- Media matching and user-confirmed resource linking.
- Annotation storage and management.
- Backup and restore.
- Media engine and reader engine interfaces.
- Tauri command and event contracts.

Platform-specific implementations must expose stable interfaces to shared Rust code.

## 2. Linux

### 2.1 Distribution

- Primary target: Fedora-compatible Linux systems.
- Distribution format: Flatpak.
- The sandboxed Flatpak build is the reference environment.
- Development builds may run outside the sandbox, but must not rely on unrestricted filesystem access in production.

### 2.2 Filesystem Access

Flatpak sandboxing restricts direct filesystem access.

- Use XDG Desktop Portals for file and folder selection.
- Use the file-chooser portal for individual files and folders.
- Persist access to user-selected locations where supported.
- Keep filesystem operations behind the Linux platform adapter.
- Do not assume arbitrary host filesystem paths are accessible.
- A non-Flatpak development build may use direct paths, but this must not become a production dependency.

### 2.3 File and Folder Selection

Use portal-backed file and folder pickers through Tauri or a suitable dedicated Rust crate.

The platform adapter must:

- Return selected resources to the shared resource-management layer.
- Preserve access to selected files where portal permissions allow.
- Report inaccessible or revoked resources.
- Avoid copying or moving user files.
- Handle cancellation without treating it as an application error.

### 2.4 Folder Scanning

Recursive scanning is supported.

- Scanning runs asynchronously.
- Large scans report progress through the shared event contract.
- Inaccessible files and unsupported formats are reported without terminating the entire scan.
- Repeated scans must not create duplicate resource records.
- Manual scanning remains available at all times.

### 2.5 Folder Monitoring

Folder monitoring is optional.

- Use filesystem notifications through a suitable library such as `notify` where supported.
- Linux implementations may use inotify.
- Monitoring must be tested inside the Flatpak sandbox.
- If monitoring is unavailable or permissions are insufficient, scanning must continue to work.
- Startup scanning is the primary fallback.

### 2.6 GStreamer Integration

<badge color="warning">Provisional</badge>

GStreamer is the proposed Linux video engine. It must pass a prototype before the choice is considered settled.

Validate:

- Video rendering to the Tauri WebView surface or a native overlay.
- Audio output and audio-track enumeration.
- Subtitle-track enumeration and selection.
- External subtitle attachment.
- Seeking and playback-position reporting.
- Playback speed control.
- Volume control.
- Audio/video synchronisation offset.
- Hardware decoding where available.
- Resource access within the Flatpak sandbox.

If the prototype cannot satisfy required playback controls or rendering requirements, document the findings in `docs/DECISIONS.md` and evaluate alternatives. Do not silently replace the engine.

### 2.7 Flatpak Requirements

- Declare required portal interfaces in the Flatpak manifest.
- Avoid unnecessary filesystem permissions.
- Test file and folder selection inside the actual sandbox.
- Test media playback using files selected through portals.
- Document runtime and permission requirements.
- Verify that packaging does not introduce dependencies unavailable in the target runtime.

## 3. Android

### 3.1 Distribution

- Target: Android phones.
- Distribution format: APK.
- The application must respect Android lifecycle and scoped-storage restrictions.
- Shared domain logic remains in Rust wherever practical.

### 3.2 File Access: Storage Access Framework

Use Android's Storage Access Framework (SAF) for file and folder selection.

- Use the system document picker for individual files.
- Use the system document-tree picker for folders.
- Obtain persisted URI permissions for selected resources.
- Store document URIs as local resource identifiers with `platform_kind = 'content_uri'`.
- Do not assume that a content URI can be converted into a normal filesystem path.
- Do not assume unrestricted access to shared storage.
- Re-verify persisted permissions when resources are accessed.

The Android adapter must expose resource operations through the shared resource-access interface.

### 3.3 Document-Tree Scanning

Scan selected document trees using Android document APIs such as `DocumentFile` or `DocumentsContract`.

Requirements:

- Support recursive traversal where the provider permits it.
- Handle providers that do not support all document operations.
- Process large trees without blocking the UI.
- Report scan progress through the shared event contract.
- Handle inaccessible documents and revoked permissions gracefully.
- Avoid duplicate resource records across repeated scans.

Do not use direct filesystem path assumptions for SAF resources.

### 3.4 Lifecycle Management

Android may suspend or terminate the application while it is in the background.

- Persistent application data must be stored in SQLite.
- Playback and reader sessions must be recoverable or safely closed.
- Progress must be persisted regularly rather than only on application exit.
- Re-verify persisted URI permissions when returning to the application or accessing a resource.
- Update resource availability when access has been revoked.
- If passcode protection is enabled, require the passcode when the application resumes from the background according to the locking policy.
- Handle process recreation without assuming in-memory frontend state still exists.

The app must not rely on receiving a final shutdown callback to save user data.

### 3.5 Media3 Integration

<badge color="warning">Provisional</badge>

AndroidX Media3 is the proposed Android video engine. It must pass a prototype before the choice is considered settled.

Validate:

- Video playback through SAF content URIs.
- Playback rendering within the Tauri application.
- Audio-track enumeration and selection.
- Subtitle-track enumeration and selection.
- External subtitle files accessed through SAF.
- Seeking and playback-position reporting.
- Playback speed and volume controls.
- Audio/video synchronisation offset.
- Integration with the Tauri activity and lifecycle.
- Resource release when the application is paused, closed, or recreated.

If Media3 cannot satisfy required controls or integration requirements, document the findings in `docs/DECISIONS.md` and evaluate alternatives.

### 3.6 Folder Monitoring

<badge color="warning">Deferred</badge>

Folder monitoring is deferred on Android for V1.

- Do not assume `FileObserver` works across SAF document providers.
- Do not implement background monitoring as a requirement.
- Manual scanning and scan-on-startup are the supported discovery mechanisms.

### 3.7 Storage and Permissions

- Use SAF rather than broad storage permissions.
- Request only permissions required by selected functionality.
- Persist URI permissions where supported.
- Revoke persisted URI permissions when the user removes the corresponding watched folder or resource access registration.
- Do not delete original files when permissions are revoked.
- Mark inaccessible resources as unavailable rather than deleting their library entries or tracking data.

## 4. Shared vs Platform-Specific Matrix

| Capability | Shared behaviour | Linux adapter | Android adapter |
|---|---|---|---|
| File selection | Resource registration | Portal | SAF |
| Folder selection | Watched-folder registration | Portal | SAF |
| Recursive folder scan | Scan logic and progress | Filesystem access | Document-tree access |
| Folder monitoring | Optional discovery updates | Optional | Deferred |
| Persist access permissions | Resource access management | Portal permissions | Persisted URI permissions |
| Read resource bytes | Shared resource interface | Filesystem | Content URI |
| Detect resource availability | Shared availability state | Filesystem checks | URI permission/access checks |
| Video playback | Shared playback interface | GStreamer* | Media3* |
| EPUB reading | Shared reader interface | Foliate.js* | Foliate.js* |
| PDF reading | Shared reader interface | PDF.js | PDF.js |
| App-level passcode | Shared security logic | — | Lifecycle integration |
| Backup and restore | Shared backup implementation | Destination picker | Destination picker |

`*` Provisional, pending prototype validation and platform integration testing.

## 5. Platform-Specific Code Organisation

Keep platform-specific implementations under:

```text
src-tauri/src/platform/
├── mod.rs
├── linux/
│   └── mod.rs
└── android/
    └── mod.rs
```

Shared interfaces belong outside the platform-specific implementation directories.

Rules:

- Shared domain code must depend on interfaces, not platform implementations.
- Platform-specific dependencies must be isolated.
- Conditional compilation must prevent platform-only code from breaking builds on the other target.
- Platform-specific errors must be translated into the shared `AppError` contract.
- Platform adapters must not implement independent versions of shared domain rules.

This is the initial organisation. Add files only when a concrete implementation requires them.

## 6. Validation Requirements

Before considering platform functionality complete, validate it on the actual target environment.

### Linux

- [ ] Build and launch the Flatpak.
- [ ] Select individual files through the portal.
- [ ] Select folders through the portal.
- [ ] Scan selected folders recursively.
- [ ] Reopen previously selected resources.
- [ ] Handle revoked or unavailable access.
- [ ] Test optional folder monitoring inside the sandbox.
- [ ] Validate video playback and required controls.
- [ ] Validate EPUB and PDF reading.
- [ ] Export and restore a backup.

### Android

- [ ] Build and install the APK.
- [ ] Select individual files through SAF.
- [ ] Select document trees through SAF.
- [ ] Persist and re-verify URI permissions.
- [ ] Scan document trees recursively.
- [ ] Handle revoked permissions and unavailable resources.
- [ ] Test backgrounding, resuming, and process recreation.
- [ ] Validate passcode locking on resume.
- [ ] Validate video playback and required controls.
- [ ] Validate EPUB and PDF reading.
- [ ] Export and restore a backup.

Record unresolved platform limitations in the relevant documentation or an ADR.

## 7. Non-Goals for V1

- Platform-specific UI theming beyond appropriate native conventions.
- Push notifications.
- Background playback services.
- Android folder monitoring.
- Cloud storage integration.
- Remote filesystem access.
- Unrestricted filesystem access.
- Guaranteed support for every media format or codec.