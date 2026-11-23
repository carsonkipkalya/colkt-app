# ROADMAP.md — Colkt Development Roadmap

## 1. Purpose

This document defines the planned development sequence for Colkt.

The roadmap establishes implementation priorities, dependencies, milestones, and release criteria. It is intended to keep development focused on delivering a usable, reliable V1 before introducing additional features.

The roadmap is organised around development phases rather than calendar dates. A phase is considered complete when its requirements and acceptance criteria have been met.

The roadmap must remain consistent with:

- `AGENTS.md`
- `docs/ARCHITECTURE.md`
- `docs/DATABASE.md`
- `docs/FEATURES.md`
- `docs/API.md`
- `docs/PLATFORM.md`
- `docs/MEDIA.md`
- `docs/SECURITY.md`

If implementation reveals that an approved architectural decision is impractical, the relevant documentation must be reviewed and updated before the implementation direction is changed.

---

## 2. Development Principles

Development must follow these principles:

1. **V1 first:** Prioritise a complete, dependable personal media library over a large collection of unfinished features.
2. **Offline-first:** Core library functionality must work without an internet connection.
3. **Local ownership:** Users retain control of their original media files. Colkt must not automatically move, rename, modify, or delete them.
4. **Cross-platform consistency:** Linux and Android should share the same core domain behaviour wherever practical.
5. **Platform-aware implementation:** File access, playback, permissions, and application lifecycle behaviour may differ between platforms.
6. **Data integrity:** Database operations, tracking, resource relationships, and backups must preserve user data.
7. **Explicit user control:** Metadata matches, resource associations, and destructive operations must not be applied without appropriate user confirmation.
8. **Incremental validation:** Each major subsystem must be tested before dependent features are built on top of it.

---

## 3. Phase 0 — Project Foundation

### Objective

Establish the development environment, application structure, and foundational engineering practices.

### Scope

- Set up the Tauri 2 application.
- Configure React, TypeScript, Vite, and Rust.
- Establish the frontend and backend directory structures.
- Configure development and production build commands.
- Establish formatting, linting, and type-checking.
- Configure Git and project-level ignore rules.
- Establish application configuration and environment handling.
- Define the frontend-to-Rust communication conventions.
- Establish a basic testing strategy for frontend and backend code.

### Platform preparation

- Confirm that the application builds and launches on Linux.
- Prepare the Android development environment.
- Confirm the Android target builds.
- Identify platform-specific dependencies and integration requirements.

### Completion criteria

- The application launches successfully on Linux.
- The frontend and Rust backend communicate successfully.
- Type-checking and linting work.
- Git tracks the project without including generated build artefacts or dependencies.
- The Android build environment is functional.
- The project structure follows the architecture document.

---

## 4. Phase 1 — Core Application and Data Layer

### Objective

Build the application foundation, persistent database, profile, and security system.

### Scope

#### Application foundation

- Implement the main application shell.
- Establish navigation and shared layout components.
- Create responsive layouts for desktop and mobile.
- Establish shared design tokens and CSS conventions.
- Implement application startup and shutdown behaviour.

#### Database

- Implement SQLite database initialisation.
- Create the schema defined in `docs/DATABASE.md`.
- Implement schema versioning and migrations.
- Configure foreign-key enforcement.
- Establish database transaction handling.
- Implement database access exclusively through the Rust backend.
- Add indexes and full-text search infrastructure where specified.

#### Profile

- Create the default profile.
- Allow editing the profile name.
- Support changing the profile picture.
- Persist profile settings locally.

#### Security

- Implement passcode setup and verification.
- Store only a secure Argon2id password hash.
- Implement application locking and unlocking.
- Ensure passcodes are never exposed to frontend persistence or ordinary logs.
- Establish the required Tauri permissions and security boundaries.

### Completion criteria

- The database initialises reliably.
- Migrations can be applied safely.
- Profile information persists across application restarts.
- Passcode verification works.
- The application can lock and unlock correctly.
- Database operations are performed through the backend.
- Core application behaviour works offline.

---

## 5. Phase 2 — Library Management

### Objective

Implement the unified library for movies, TV shows, and books.

### Scope

#### Media entries

- Create, view, edit, and delete library entries.
- Support movies, TV shows, and books.
- Implement media-specific information and fields.
- Support manual metadata entry.
- Support external identifiers.
- Prevent duplicate external identifiers where required.
- Implement library listing and detail views.

#### TV structure

- Implement seasons and episodes.
- Support episode ordering and season relationships.
- Display TV show progress and episode information.

#### Crates

- Create and edit user-defined crates.
- Delete crates without deleting their media entries.
- Add and remove library entries from crates.
- Support crate media-type restrictions.
- Display crate contents.

#### Search and filtering

- Implement local library search.
- Add filtering by media type.
- Support relevant sorting and browsing options.
- Implement database-backed search using the defined search infrastructure.

### Completion criteria

- Users can manage movies, TV shows, and books.
- Media entries persist correctly.
- TV seasons and episodes maintain their relationships.
- Crates work independently of the underlying media entries.
- Search and filtering work without an internet connection.
- Database constraints prevent invalid relationships and values.

---

## 6. Phase 3 — Tracking and Library Activity

### Objective

Implement personal progress and activity tracking for all supported media types.

### Scope

#### Movies

- Mark movies as watched or unwatched.
- Record watched dates where applicable.
- Support ratings on a 10-point scale.
- Persist tracking information.

#### TV shows

- Track watched and unwatched episodes.
- Support episode-level watched dates.
- Derive show and season progress from episode tracking.
- Support show-level tracking and ratings.
- Ensure changes to episode tracking update derived progress correctly.

#### Books

- Track reading progress.
- Support physical and digital reading formats.
- Support page-based or percentage-based progress where applicable.
- Support 5-star ratings.
- Distinguish physical book progress from digital reading progress.

### Completion criteria

- Tracking information persists across restarts.
- Watched states and reading progress remain consistent.
- Derived TV progress reflects episode-level activity.
- Ratings enforce their defined ranges.
- Tracking data remains separate from local file availability.

---

## 7. Phase 4 — Local Media and Resource Management

### Objective

Allow users to connect their existing media files to library entries without taking ownership of or modifying those files.

### Scope

#### Resource management

- Implement local resource registration.
- Support multiple resources associated with one library entry.
- Support primary resources, additional resources, and subtitles where applicable.
- Allow users to link and unlink resources.
- Track resource availability.
- Handle missing or inaccessible resources.

#### Watched folders

- Allow users to select folders for media discovery.
- Implement recursive folder scanning where supported.
- Register discovered resources.
- Avoid creating library entries automatically from discovered files.
- Support rescanning and discovery of changed files.
- Implement the explicit watched-folder-to-resource mapping.

#### File access

- Implement Linux file and folder access using appropriate permissions and portals.
- Implement Android Storage Access Framework integration.
- Persist Android URI permissions where supported.
- Handle revoked permissions and inaccessible resources.

#### Media identification

- Extract available file information.
- Identify candidate matches against existing library entries.
- Present possible matches to the user.
- Require user confirmation before linking a resource to an existing entry.
- Allow users to create a new library entry when appropriate.

#### Monitoring

- Implement startup scanning.
- Support optional Linux folder monitoring where practical.
- Defer continuous Android folder monitoring unless a reliable platform-compatible approach is established.

### Completion criteria

- Users can register existing media files.
- Users can scan supported folders.
- Discovered files do not automatically create library entries.
- Candidate matches require confirmation.
- Multiple resources can be associated with one entry.
- Unlinking a resource does not delete the original file or library entry.
- Revoked permissions and missing files are handled safely.

---

## 8. Phase 5 — Metadata Integration

### Objective

Provide optional online metadata identification and retrieval while preserving offline functionality.

### Scope

#### Providers

- Integrate TMDB for movie and TV metadata.
- Integrate TVmaze for TV metadata.
- Integrate Open Library for book metadata.

#### Metadata operations

- Search external providers.
- Retrieve relevant metadata.
- Present candidate matches.
- Allow users to confirm or reject matches.
- Store external identifiers.
- Cache retrieved metadata locally.
- Handle provider errors, rate limits, and unavailable services.
- Respect provider terms, licensing requirements, and attribution rules.

#### Offline behaviour

- Keep existing metadata available offline.
- Allow manual entry and editing without network access.
- Avoid making metadata providers mandatory for library operations.
- Ensure network failures do not prevent access to existing library data.

### Completion criteria

- Metadata can be searched and retrieved from supported providers.
- Users remain in control of matching and linking.
- Cached metadata remains available offline.
- Provider failures degrade gracefully.
- No local media files or unnecessary private user information are uploaded to metadata providers.

---

## 9. Phase 6 — Video Playback

### Objective

Allow users to play supported local video files from within Colkt.

### Scope

#### Playback integration

- Validate the proposed Linux GStreamer integration.
- Validate the proposed Android Media3 integration.
- Implement a shared playback interface.
- Implement platform-specific playback engines.
- Support opening a linked local resource.
- Implement playback controls.
- Support seeking and playback position.
- Persist playback state where appropriate.
- Handle unsupported formats and playback errors.
- Support subtitle resources where practical.

#### Resource handling

- Ensure playback uses approved local resources.
- Handle missing files and revoked access permissions.
- Ensure playback does not modify original media files.
- Handle playback interruption and application lifecycle changes.

### Completion criteria

- Supported video files play inside the application.
- Playback controls work.
- Playback state is persisted correctly.
- Errors are communicated clearly.
- Linux and Android use their respective validated playback integrations.
- The original media files remain unchanged.

---

## 10. Phase 7 — Book Reading

### Objective

Implement in-app reading for supported EPUB and PDF documents.

### Scope

#### Reader integration

- Validate Foliate.js for EPUB reading.
- Validate Mozilla PDF.js for PDF reading.
- Implement a shared reader interface.
- Support opening linked book resources.
- Implement reading position persistence.
- Restore reading position when reopening a book.

#### Annotations

- Support highlights.
- Support standalone notes.
- Support notes attached to highlights.
- Persist annotation information.
- Store EPUB and PDF selectors using the appropriate format.
- Handle annotations whose original text or page location can no longer be resolved.
- Allow users to review stale or unresolved annotations.

#### Resource handling

- Ensure readers access files through the approved resource-access mechanism.
- Handle missing files and revoked permissions.
- Treat imported documents and their contents as untrusted input.

### Completion criteria

- Supported EPUB and PDF files open in the application.
- Reading position persists.
- Highlights and notes can be created and retrieved.
- Annotation relationships remain valid.
- Missing resources and invalid annotations are handled safely.

---

## 11. Phase 8 — Backup and Restore

### Objective

Protect user-created library data and provide a reliable recovery mechanism.

### Scope

- Define the backup format.
- Back up the SQLite database consistently.
- Account for SQLite WAL and journal behaviour.
- Include relevant application settings and metadata cache where appropriate.
- Preserve library entries, tracking, crates, annotations, and resource references.
- Implement backup validation.
- Implement restore operations.
- Prevent archive path traversal and unsafe extraction.
- Handle incompatible backup versions.
- Report backup and restore progress.
- Explain that original media files are not copied into backups unless a future feature explicitly introduces that behaviour.

### Completion criteria

- Users can create backups.
- Backups contain the supported application data.
- Users can restore a valid backup.
- Invalid or incompatible backups fail safely.
- Restoring a backup does not unexpectedly modify or delete original media files.
- Backup and restore behaviour is tested on Linux and Android.

---

## 12. Phase 9 — Testing, Hardening, and V1 Release

### Objective

Validate the complete application, fix critical issues, and prepare reliable Linux and Android releases.

### Scope

#### Functional testing

- Test profile creation and editing.
- Test passcode setup, verification, locking, and unlocking.
- Test library entry management.
- Test crates and search.
- Test tracking and progress calculations.
- Test watched-folder scanning.
- Test resource linking and unlinking.
- Test metadata retrieval and offline behaviour.
- Test video playback.
- Test EPUB and PDF reading.
- Test annotations.
- Test backup and restore.

#### Data integrity

- Test database migrations.
- Test transaction rollback.
- Test foreign-key constraints.
- Test missing resources.
- Test revoked permissions.
- Test interrupted operations.
- Test application restart and recovery.

#### Security

- Review Tauri capabilities and permissions.
- Review Content Security Policy.
- Validate backend input handling.
- Test passcode handling and lock behaviour.
- Review logging for sensitive information.
- Test backup archive validation.
- Review dependency security.
- Verify that the application does not modify original media files.

#### Platform testing

**Linux**

- Validate the application on Fedora.
- Validate Flatpak packaging.
- Test file and folder access.
- Test playback integration.
- Test application startup, shutdown, and locking.

**Android**

- Validate APK packaging.
- Test installation and application lifecycle.
- Test Storage Access Framework permissions.
- Test media playback.
- Test reading.
- Test lock and resume behaviour.

#### Release preparation

- Prepare application icons and branding.
- Configure release versioning.
- Build production assets.
- Produce Linux Flatpak packages.
- Produce Android APK builds.
- Prepare installation and user documentation.
- Publish source code and release artefacts through an appropriate public repository or distribution channel.

### V1 completion criteria

Colkt V1 is ready when:

- The core application works reliably on Linux and Android.
- Users can maintain a unified library of movies, TV shows, and books.
- Users can organise entries into crates.
- Users can track watching and reading progress.
- Users can connect existing local media files.
- Media identification requires user confirmation.
- Supported video files can be played.
- Supported EPUB and PDF files can be read.
- Highlights and notes can be stored.
- Backup and restore work reliably.
- Core library functionality works offline.
- Original media files are not modified or deleted by the application.
- Critical security, data integrity, and platform compatibility issues have been resolved.

---

## 13. Post-V1 Possibilities

The following features are outside the initial V1 commitment. They may be considered after V1 has been released and its stability has been established.

### Library and profiles

- Multiple user profiles.
- Profile-specific libraries and tracking.
- More advanced library organisation.
- Additional crate capabilities.

### Media discovery

- More metadata providers.
- Improved matching and identification.
- Optional metadata refresh.
- More advanced media discovery tools.

### Reading and playback

- Additional media formats.
- Improved subtitle support.
- More advanced playback controls.
- Additional annotation and reading tools.
- OCR and document text extraction.

### Platform and storage

- Improved Android folder monitoring.
- Additional Linux packaging options.
- More advanced resource availability monitoring.
- External storage improvements.

### Synchronisation and connectivity

- Optional local-network synchronisation.
- Optional remote synchronisation.
- Additional backup destinations.

These possibilities are not commitments. Each must be evaluated for feasibility, privacy, security, maintenance cost, and compatibility with Colkt's offline-first principles.

---

## 14. Explicit V1 Non-Goals

The following are not part of the initial V1 release:

- Cloud accounts or mandatory online authentication.
- Cloud-hosted library storage.
- Cloud synchronisation.
- Remote media streaming.
- Multiple user profiles.
- Social features.
- Recommendations and personalised discovery systems.
- OCR and full-text extraction from documents.
- Automatic metadata matching without user confirmation.
- Automatic modification, movement, renaming, or deletion of original media files.
- Mandatory online access for core library operations.

Any change to these exclusions requires an explicit scope decision and corresponding documentation updates.

---

## 15. Roadmap Management

This roadmap is a development guide, not a rigid implementation contract.

The following rules apply:

1. Complete and validate foundational work before implementing dependent features.
2. Do not mark a phase complete merely because its interface exists.
3. Backend behaviour, database persistence, error handling, and tests must be included in completion assessment.
4. Platform-specific features must be validated on their target platforms.
5. Provisional technologies must be tested before being treated as permanent architectural decisions.
6. Changes to scope must be reflected in the relevant documentation.
7. Security and data-integrity requirements take priority over convenience.
8. Features deferred from V1 must not silently enter the V1 implementation.

When a phase is completed, record its status in the project's development tracking system or Git history.

The roadmap should be reviewed whenever a major architectural decision changes or the V1 scope is formally revised.