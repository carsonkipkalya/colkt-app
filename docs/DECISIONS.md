# DECISIONS.md — Colkt Architecture Decision Records

## 1. Purpose

This document records significant architectural and technical decisions made during Colkt development.

Each Architecture Decision Record (ADR) captures:

- The context and problem.
- The decision made.
- Alternatives considered.
- Consequences and trade-offs.
- The current decision status.

The purpose is to prevent undocumented architectural changes, preserve the reasoning behind important choices, and provide a reliable reference for future development.

## 2. Decision Statuses

Decisions use the following statuses:

- **Accepted:** The decision is approved and should be followed.
- **Accepted — Provisional Implementation:** The architectural direction is approved, but the selected implementation requires prototype validation.
- **Proposed:** A decision is being considered but has not been approved.
- **Superseded:** A newer ADR has replaced the decision.
- **Rejected:** The option was considered and explicitly rejected.

A provisional implementation must not be treated as fully validated merely because its architectural direction has been accepted.

---

## ADR-001 — Tauri 2, React, TypeScript, and Vite

**Status:** Accepted.

### Context

Colkt is a private, offline-first personal media library targeting Linux desktop and Android.

The project needs a shared frontend, native platform integration, local data storage, and access to platform-specific media capabilities.

### Decision

Use:

- Tauri 2 as the application framework.
- React for the frontend.
- TypeScript for frontend development.
- Vite for frontend tooling.
- Rust for the native backend.

### Alternatives considered

- **Electron:** Rejected because of its larger runtime footprint and less suitable resource profile.
- **Separate native applications:** Rejected because maintaining independent applications would increase development and maintenance work.
- **A web-only application:** Rejected because Colkt requires native filesystem access, local database ownership, and platform-specific media integration.

### Consequences

- Linux and Android share the main application architecture.
- Rust provides the backend and platform integration layer.
- The frontend can use web technologies for the interface and reader integrations.
- Platform-specific functionality must be isolated behind appropriate interfaces.

---

## ADR-002 — Rust Backend Owns SQLite

**Status:** Accepted.

### Context

Colkt requires consistent database access, migrations, validation, and domain behaviour across Linux and Android.

Allowing the frontend to access SQLite directly would weaken the architectural boundary and risk inconsistent business rules.

### Decision

The Rust backend exclusively owns:

- SQLite connections.
- Database migrations.
- Database queries and writes.
- Transaction management.
- Domain validation.
- Persistent application state.

The React frontend communicates with the backend exclusively through Tauri commands and events defined in `docs/API.md`.

### Alternatives considered

- **Frontend SQLite access through a Tauri SQL plugin:** Rejected because it would expose database operations to the presentation layer.
- **A local backend server:** Rejected because Colkt does not require a separate server process.

### Consequences

- Database access remains centralised.
- Domain rules are implemented consistently.
- The frontend depends on defined IPC contracts rather than database details.
- Database migrations are managed by Rust.
- API changes must be reflected in `docs/API.md` and the corresponding frontend types.

---

## ADR-003 — Local-First Architecture, No Server or Cloud in V1

**Status:** Accepted.

### Context

Colkt is intended to be a private personal media library.

Core functionality should remain available without an internet connection, and users should not need online accounts or cloud infrastructure to manage their libraries.

### Decision

V1 operates locally without:

- A backend server.
- Mandatory online accounts.
- Cloud-hosted library storage.
- Cloud synchronisation.

Online access is limited to optional metadata and catalogue searches initiated by the user.

### Alternatives considered

- **Cloud-backed library:** Deferred because it introduces account management, hosting costs, privacy concerns, and synchronisation complexity.
- **Self-hosted server:** Rejected for V1 because it introduces additional installation and maintenance requirements.
- **Mandatory online metadata services:** Rejected because core library functionality must work offline.

### Consequences

- The local SQLite database is the source of truth.
- Metadata services are optional.
- Cached metadata remains available offline.
- Network failures must not prevent access to existing library data.
- Synchronisation conflict handling is outside V1 scope.

---

## ADR-004 — Unified Media Item with Type-Specific Extensions

**Status:** Accepted.

### Context

Movies, TV shows, and books share common library functionality, including:

- Listing and searching.
- Crate membership.
- Local resource linking.
- Common metadata fields.

However, their detailed metadata and tracking requirements differ.

### Decision

Use a unified `media_item` table with a `media_type` discriminator.

Type-specific data is stored in extension tables:

- `movie`
- `tv_show`
- `book_work`

Books use a work-and-edition model:

- A book work represents the library entry.
- Editions represent specific publications with their own ISBN, publisher, page count, and other edition-specific information.
- Tracking and ratings belong to the book work.

### Alternatives considered

- **Three independent top-level media tables:** Rejected because unified search, crates, and resource linking would become more complicated.
- **One large table containing every possible field:** Rejected because it would create sparse records and make the schema harder to maintain.

### Consequences

- Common library operations work across all media types.
- Type-specific data remains separated.
- Crates and resource links reference `media_item`.
- Book tracking remains associated with the work rather than a particular edition.
- V1 does not attempt to reconcile page numbers between different editions.

---

## ADR-005 — Video Media Engine Abstraction

**Status:** Accepted — Provisional Implementation.

### Context

Colkt must support local video playback on Linux and Android.

The platforms have different media capabilities, native APIs, and lifecycle requirements.

The domain layer should not depend directly on a particular playback engine.

### Decision

Define a shared `MediaEngine` interface.

The initial platform implementations are:

- GStreamer on Linux.
- Android Media3 on Android.

The interface should cover the common playback operations required by Colkt, including:

- Opening a resource.
- Playing and pausing.
- Seeking.
- Playback speed.
- Volume.
- Track selection where supported.
- Subtitle attachment where supported.
- Playback position.
- Playback events and errors.

Engine-specific types must remain inside their respective implementation modules.

### Alternatives considered

- **Direct engine calls from domain code:** Rejected because it would couple business logic to platform-specific implementations.
- **One cross-platform playback engine:** Not selected because no suitable implementation has yet been validated against the required platform capabilities.

### Open questions requiring prototypes

1. Can GStreamer render reliably inside the Tauri WebView on Linux?
2. Can Android Media3 integrate reliably with the Tauri Android activity and application lifecycle?
3. Can both implementations provide the required playback controls?
4. How reliably can audio and subtitle tracks be enumerated and selected?
5. Can external subtitles be attached?
6. Are audio/video and subtitle synchronisation controls available?
7. Can playback position be reported frequently enough without excessive database writes?

### Consequences

- The application has a shared playback contract.
- Platform engines can be replaced without changing domain-level tracking rules.
- Playback capabilities may differ between platforms.
- Unsupported controls must be communicated honestly.
- Engine limitations must be documented in `docs/MEDIA.md`.

### Validation milestone

Prototype and validate the Linux and Android implementations during **Phase 6 — Video Playback** in `docs/ROADMAP.md`.

Do not mark video playback complete until the required capabilities have been tested on their respective platforms.

---

## ADR-006 — Book Reader Engine Abstraction

**Status:** Accepted — Provisional Implementation.

### Context

Colkt needs in-app EPUB and PDF reading.

The proposed reader technologies are web-based and should remain replaceable without coupling their internal data structures to the database or domain layer.

### Decision

Use a shared reader integration boundary.

The initial implementations are:

- Foliate.js for EPUB.
- Mozilla PDF.js for PDF.

The reader engines run through frontend adapters inside the application WebView.

The frontend handles reader presentation and engine interaction. The backend remains responsible for validating and persisting reading progress and annotation data through the defined commands.

Engine-specific selection data must be converted into the stable selector formats defined in `docs/DATABASE.md`.

### Alternatives considered

- **Native reader implementations:** Not selected for V1 because they would increase cross-platform implementation complexity.
- **Persisting engine-specific annotation structures directly:** Rejected because it would couple stored data to a particular reader implementation.

### Open questions requiring prototypes

1. How reliably do EPUB CFI selectors survive reopening and changes to reader state?
2. How robust are annotations across EPUB variants?
3. How well does PDF.js render large documents on Android?
4. What is the performance of PDF annotation rendering on lower-end devices?
5. Can reader selection and progress events be integrated reliably with backend persistence?

### Consequences

- Reader engines remain replaceable.
- Annotation persistence is independent of the reader implementation.
- EPUB and PDF use different selector formats.
- Changed or replaced documents may invalidate annotation anchors.
- Invalid or stale annotations must be flagged for user review rather than silently relocated.

### Validation milestone

Prototype and validate the reader integrations during **Phase 7 — Book Reading** in `docs/ROADMAP.md`.

---

## ADR-007 — Local File Access Strategy

**Status:** Accepted — Android Folder Monitoring Deferred.

### Context

Linux Flatpak and Android have different filesystem access and permission models.

Colkt must access user-owned media without copying it into application storage or modifying the original files.

### Decision

Use platform-specific file access implementations behind a shared platform interface.

**Linux:**

- Use native file and folder selection through portals or supported Tauri integrations.
- Use permitted filesystem paths for registered resources.
- Support optional folder monitoring where sandbox permissions allow.
- Use inotify-based monitoring where appropriate and validated.

**Android:**

- Use the Storage Access Framework (SAF).
- Support file and document-tree selection.
- Persist URI permissions where Android allows.
- Use document-tree scanning for selected folders.
- Do not assume unrestricted filesystem paths are available.

All registered media resources are represented through `local_resource` records.

### Alternatives considered

- **Raw filesystem paths on Android:** Rejected because Android scoped storage does not guarantee unrestricted path access.
- **Copying media into application storage:** Rejected because it violates Colkt's user-owned media principle.
- **One shared filesystem implementation:** Rejected because the platforms expose fundamentally different storage capabilities.

### Consequences

- Resource identifiers are platform-specific.
- Linux resources use `file_path`.
- Android resources use `content_uri`.
- Resource access is mediated by the backend and platform adapters.
- Missing files and revoked permissions must be handled without deleting associated library entries.
- Android folder monitoring is not required for V1.

### Validation milestone

Validate Linux portals, Flatpak permissions, Android SAF, and persisted URI permissions during **Phase 4 — Local Media and Resource Management**.

---

## ADR-008 — Annotation Storage

**Status:** Accepted.

### Context

Highlights and notes must persist across application restarts and must not modify original EPUB or PDF files.

Annotations also need a stable representation that is not tied to one reader engine.

### Decision

Store annotations in SQLite.

Each annotation contains a format-specific selector:

**EPUB:**

- EPUB CFI.
- Text quote.
- Optional chapter context.

**PDF:**

- Page number.
- Normalised rectangle coordinates.
- Text quote.

Annotations are associated with the relevant book work and may reference a specific edition and local resource.

Notes can be stored independently or attached to highlights according to the annotation model defined in `docs/DATABASE.md`.

### Alternatives considered

- **Embedding annotations inside book files:** Rejected because it modifies user-owned files.
- **Sidecar annotation files:** Rejected because they introduce additional file management and synchronisation problems.

### Consequences

- Original book files remain unchanged.
- Annotations are stored independently of the reader.
- EPUB and PDF use different selector structures.
- Changed files may invalidate annotation anchors.
- Stale annotations must be identified and presented for review.
- Annotation behaviour must remain consistent with `docs/DATABASE.md` and `docs/API.md`.

---

## ADR-009 — Backup Format

**Status:** Accepted.

### Context

Users need a portable way to protect their Colkt library data.

Backups must be versioned and support safe restoration even as the database schema evolves.

Original media files should not be duplicated by default.

### Decision

Use a single ZIP archive containing:

- A manifest.
- Application and backup format versions.
- Database schema version.
- Checksums for backup content.
- A JSONL export of application database tables.

Original media files are excluded by default.

Local resource references are included so restored library entries can retain their associations. Resources that cannot be accessed after restoration must be marked unavailable rather than silently removed.

Restore must support the merge and replace behaviours defined in `docs/FEATURES.md`.

### Alternatives considered

- **Copying the SQLite database file directly:** Not selected because it is less flexible for schema migration and cross-version restoration.
- **A custom binary format:** Rejected because it adds unnecessary complexity.

### Consequences

- Backups are inspectable and versioned.
- Restore can validate compatibility before importing data.
- Database changes can be handled through explicit backup-version migration logic.
- Restore operations must be transactional.
- Archive extraction must prevent path traversal and unsafe file writes.
- Original media files remain outside the backup by default.

### Validation milestone

Validate backup creation, compatibility checks, merge/replace behaviour, and restoration during **Phase 8 — Backup and Restore**.

---

## ADR-010 — Passcode Is an Application Access Gate, Not Encryption

**Status:** Accepted.

### Context

Users may assume that setting an application passcode encrypts the underlying database.

V1 requires an application access gate but does not include database encryption.

### Decision

Use an optional application-level passcode.

The passcode is stored as a salted Argon2id password hash.

The passcode gates application access on launch and unlock but does not encrypt the SQLite database or original media files.

Database encryption is deferred.

### Alternatives considered

- **SQLCipher or equivalent database encryption:** Deferred because it introduces additional dependencies, key management, recovery, and cross-platform integration requirements.
- **No passcode:** Rejected because an application-level access gate is part of the intended privacy experience.

### Consequences

- The passcode must never be stored in plaintext.
- The frontend must not persist the passcode.
- Passcode verification is performed by the backend.
- The UI must clearly explain that the passcode does not encrypt the database.
- Users requiring at-rest protection must rely on operating-system or full-disk encryption.
- Database encryption would require a separate architectural decision.

---

## ADR-011 — One Profile Per Installation in V1

**Status:** Accepted.

### Context

Colkt V1 is a personal media library without online accounts or multi-user support.

A profile is still useful for storing the user's editable identity and picture.

### Decision

Create one profile per installation.

The profile is created automatically on first launch and can be edited by the user.

V1 does not support multiple profiles.

### Alternatives considered

- **Multiple local profiles:** Deferred because they introduce profile switching, data ownership, and passcode association complexity.
- **No profile:** Rejected because profile identity and settings provide a useful foundation for the application.

### Consequences

- The database contains one default profile.
- Profile settings remain local.
- Passcode settings apply at the application level.
- Backups include profile information.
- Multi-profile support requires an explicit future decision and database migration.

---

## 3. Pending Prototypes

The following items are not fully validated implementation decisions.

They must be tested before the corresponding roadmap phases are marked complete.

### Video

**GStreamer rendering inside Tauri WebView on Linux**

Validate:

- Rendering compatibility.
- Playback controls.
- Resource access.
- Audio and subtitle track handling.
- Performance and stability.

**Media3 lifecycle integration on Android**

Validate:

- Integration with the Tauri Android activity.
- Playback lifecycle behaviour.
- Resource access through SAF.
- Playback controls.
- Track handling.
- Behaviour when the application is paused or resumed.

### Reading

**Foliate.js annotation anchoring**

Validate:

- CFI generation and resolution.
- Behaviour across EPUB variants.
- Reopening documents.
- Handling changed or replaced EPUB files.

**PDF.js annotation performance**

Validate:

- Large document rendering.
- Text selection.
- Annotation overlays.
- Performance on Android WebView.
- Behaviour on lower-end devices.

### Resource discovery

**Detected but unconfirmed resource status**

The current design treats matching suggestions as transient until the user confirms a resource association.

A persistent detected-but-unconfirmed state is not part of the current database schema.

Revisit this only if practical testing shows that resumable scans or persistent candidate review require it.

Any change must be documented and reflected in `docs/DATABASE.md`, `docs/API.md`, and `docs/FEATURES.md`.

---

## 4. Decision-Making and Change Management

Architectural decisions must not be changed silently.

The following changes require a new ADR and explicit user approval:

- Changing the application framework or core technology stack.
- Replacing SQLite or changing database ownership.
- Changing a major database entity or foreign-key policy.
- Changing a Tauri command signature or event contract.
- Introducing a new architectural abstraction.
- Changing the local file access strategy.
- Changing the media engine or reader engine architecture.
- Changing the backup format.
- Changing annotation storage or selector formats.
- Introducing cloud services, accounts, or synchronisation into V1.

### ADR numbering

- Assign each new ADR the next available sequential number.
- Never reuse an existing ADR number.
- Do not silently rewrite an accepted decision to reflect a new direction.
- Mark an old ADR as superseded when a new decision replaces it.
- Update affected documentation whenever a decision changes.

### ADR template

Use this template for future decisions:

```markdown
## ADR-NNN — Decision Title

**Status:** Proposed | Accepted | Accepted — Provisional Implementation | Rejected | Superseded.

### Context

Describe the problem and why a decision is necessary.

### Decision

Describe the chosen approach.

### Alternatives considered

Describe the alternatives and why they were not selected.

### Consequences

Describe the benefits, limitations, and trade-offs.

### Validation requirements

List prototypes or tests required before implementation is considered complete.
```

---

## 5. Related Documentation

Architectural decisions must remain consistent with:

- `AGENTS.md` — Instructions for AI coding agents.
- `docs/ARCHITECTURE.md` — System architecture and dependency boundaries.
- `docs/DATABASE.md` — Database schema and relationships.
- `docs/FEATURES.md` — V1 feature requirements and acceptance criteria.
- `docs/API.md` — Frontend/backend commands and event contracts.
- `docs/PLATFORM.md` — Linux and Android platform behaviour.
- `docs/MEDIA.md` — Playback and reader integrations.
- `docs/SECURITY.md` — Security and privacy requirements.
- `docs/ROADMAP.md` — Development phases and milestones.
- `docs/DECISIONS.md` — Architectural decisions and their history.