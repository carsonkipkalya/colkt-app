# AGENTS.md — Colkt Project Instructions

## 1. Project Purpose

Colkt is a private, offline-first personal media library for Linux desktop and Android.

It helps users organise movies, TV shows and books in one library, create custom collections called crates, track watching and reading progress, and access supported local media inside the application.

Colkt is a locally installed application. It does not require a browser, application server, cloud database or online account.

## 2. Technology Stack

Do not silently replace or introduce alternative technologies.

| Area | Technology |
|---|---|
| Application framework | Tauri 2 |
| Frontend | React + TypeScript |
| Frontend tooling | Vite |
| Styling | CSS Modules + CSS custom properties/design tokens |
| Backend | Rust |
| Database | SQLite, accessed through Rust |
| Linux video | GStreamer (provisional) |
| Android video | Android Media3 (provisional) |
| EPUB reader | Foliate.js (provisional) |
| PDF reader | Mozilla PDF.js |
| Linux distribution | Flatpak initially |
| Android distribution | APK initially |
| Version control | Git and GitHub |

The media engines and reader integrations remain provisional until their compatibility and required functionality have been validated.

Changes to established architectural choices require an Architecture Decision Record (ADR) in `docs/DECISIONS.md` and explicit user approval.

Do not add alternative frameworks or libraries simply because they are available.

## 3. Core Product Principles

1. Offline-first: core library, tracking, crate and reading features must work without internet access.
2. Privacy: user data remains on the device unless the user explicitly initiates an external service request.
3. No mandatory account, cloud service or application server in V1.
4. Original media files must never be copied, moved, renamed or deleted by Colkt.
5. Local media access must use references to the original resources.
6. Fuzzy media matching must never automatically merge entries or link files. User confirmation is required.
7. Online metadata must enhance the application without becoming a requirement for core functionality.
8. Prefer simple, maintainable implementations over unnecessary abstraction.

## 4. Architectural Rules

### Backend and frontend responsibilities

1. Rust owns the database, persistence and domain logic.
2. React must never access SQLite directly.
3. The frontend communicates with Rust through documented Tauri commands and events.
4. Command and event contracts must be documented in `docs/API.md`.
5. Domain rules belong in Rust. React handles presentation, navigation, interaction and frontend-specific state.
6. Validate command inputs in the backend. Never trust frontend validation alone.
7. Keep frontend and backend types consistent across the IPC boundary.

### Application structure

1. Keep domain logic independent of platform-specific implementations.
2. Platform-specific functionality must be isolated behind appropriate interfaces or adapters.
3. Video and reader engines must be replaceable without changing core domain logic.
4. Engine-specific types must not leak into shared domain models or frontend contracts.
5. Do not introduce abstraction layers without a concrete requirement.
6. Do not assume that Linux and Android have identical filesystem, playback or reader capabilities.

### Database ownership

Use this flow:

React frontend → Tauri command → Rust application service → repository → SQLite.

The frontend must not execute SQL or manipulate database files.

### File access

1. Linux file access uses Rust filesystem operations and appropriate file/folder selection mechanisms.
2. Android file access uses Storage Access Framework (SAF) and persisted URI permissions.
3. Never assume an Android content URI corresponds to an ordinary filesystem path.
4. Resolve and validate local resources in Rust.
5. Use resource identifiers rather than trusting arbitrary frontend-supplied filesystem paths.
6. Large media files must not be transferred through Tauri IPC as whole-file Base64 strings.
7. Use appropriate native or bounded range-based access for large files.

## 5. Domain Rules

### Library and local media

These are separate concepts:

- Library entries represent titles the user wants to organise or track.
- Local resources represent files accessible on the device.
- Resource links associate local files with library entries.

A title can exist in the library without a local file.

A local file can be discovered without automatically becoming a library entry.

A local file can be linked to an existing library entry only after user confirmation.

Removing a local file or losing access to it must not automatically delete its library entry or tracking information.

### Crates and tracking

1. Crates are user-defined collections.
2. Library membership, crate membership and local availability are independent.
3. Watching and reading progress must be preserved when local resources are linked or removed.
4. Movie and TV ratings use a 10-point scale.
5. Book ratings use a 5-star scale.
6. TV tracking supports season and episode-level progress.
7. Physical-book progress must remain distinct from digital-book progress.

### Reading and annotations

1. EPUB and PDF annotations belong to Colkt, not the rendering engines.
2. Annotations and reading positions must persist in SQLite.
3. EPUB annotations must use stable document-location anchors, not rendered page numbers alone.
4. PDF annotations must use document identity, page indices and page-relative coordinates.
5. Reflow, zoom and rotation must not silently corrupt annotation locations.
6. Original book files must remain untouched.

## 6. Coding Conventions

### TypeScript

- Keep strict mode enabled.
- Avoid `any` in shared code.
- Prefer `unknown` with appropriate type guards.
- Use `camelCase` for variables, functions and IPC payload properties.
- Use `PascalCase` for React components and TypeScript types.
- Use named exports for React features unless a framework or library requires otherwise.
- Keep components focused and reasonably small.

### CSS

- Use CSS Modules for component-level styling.
- Use CSS custom properties for shared design tokens.
- Avoid CSS-in-JS unless explicitly approved.
- Use `kebab-case` for CSS class names.
- Avoid duplicating shared design values.

### Rust

- Follow Rust naming conventions.
- Use `snake_case` for functions, variables and modules.
- Use `PascalCase` for types.
- Run `cargo fmt` and `cargo clippy` when appropriate.
- Use the project's configured Rust edition (currently 2024).
- Handle expected errors explicitly rather than panicking.

### General

- Keep functions focused and named after the domain concepts they implement.
- Avoid unnecessary utility layers, generic frameworks and premature abstractions.
- Do not create barrel files unnecessarily.
- Follow the existing project structure rather than reorganising it without a reason.

## 7. Dependency Rules

1. Do not introduce runtime dependencies without explaining their purpose and obtaining user approval.
2. Architectural dependencies must be documented in `docs/DECISIONS.md`.
3. Prefer existing project dependencies and platform capabilities.
4. Check licensing and maintenance status before introducing third-party libraries.
5. Avoid duplicate libraries that solve an existing problem.
6. Do not replace established technologies without approval.

## 8. Database Rules

1. Every schema change requires a new numbered migration under `src-tauri/migrations/`.
2. Migrations are append-only. Never edit a migration that has already been applied.
3. Enable and enforce SQLite foreign keys.
4. Document foreign-key cascade and restrict behaviour.
5. Wrap related multi-step writes in transactions.
6. Use prepared statements and parameterised queries.
7. Update `docs/DATABASE.md` and the ER diagram whenever the schema changes.
8. Preserve user data during migrations.
9. Backups must use a consistent SQLite snapshot rather than copying an active database file without accounting for its journal or WAL.

## 9. Security and Privacy Rules

1. Never expose arbitrary filesystem, SQL or shell execution capabilities to the frontend.
2. Follow Tauri capability restrictions and the Content Security Policy documented in `docs/SECURITY.md`.
3. Treat imported media and metadata as untrusted input.
4. EPUB scripts must be disabled or safely isolated. Imported content must never gain access to privileged native commands.
5. Block remote resources by default unless explicitly permitted.
6. Validate external links and user-controlled input.
7. The app passcode protects access to Colkt but does not encrypt the database.
8. Store passcodes only as salted password hashes using a modern password hashing algorithm. Argon2id is preferred.
9. Apply appropriate throttling to passcode attempts.
10. Never log passcodes, user content or sensitive file paths in production.
11. Do not automatically transmit scanned filenames or local media information to metadata providers.

## 10. Error Handling

1. Backend operations must return structured, serialisable errors.
2. Use consistent error codes and messages.
3. Frontend components must display understandable messages rather than exposing raw internal errors.
4. Never panic in command handlers for expected error conditions.
5. Log enough diagnostic information to investigate failures without exposing private user data.
6. Handle unavailable files, revoked Android permissions, unsupported formats and network failures gracefully.

## 11. Platform-Specific Rules

### Linux

- Prioritise Fedora compatibility during development.
- Account for Flatpak sandboxing and filesystem permissions.
- Use portals or explicitly documented permissions for file access.
- Do not assume filesystem watchers are always available.
- Validate GStreamer playback in packaged builds, not only development builds.

### Android

- Respect scoped storage.
- Use SAF for user-selected files and folders.
- Persist and validate URI permissions.
- Never construct ordinary filesystem paths from content URIs.
- Do not assume continuous folder monitoring is available.
- Account for Android lifecycle events, rotation, process recreation and audio focus.
- Validate playback and reader functionality on actual Android devices.

### Shared functionality

Keep shared domain behaviour consistent while allowing platform-specific implementations where necessary.

Never claim a format or capability is supported until it has been tested on the relevant platform.

## 12. Testing Requirements

Run relevant checks after changes.

### Rust

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

### Frontend

- `npm run lint`
- `npm run build`

Add appropriate unit, integration and component tests as the project develops.

Platform-specific integrations must be tested on their target platforms.

Document important manual test results in `docs/PLATFORM.md` or an appropriate ADR.

Do not claim tests passed if they were not actually executed.

## 13. Avoid Unnecessary Refactoring

- Implement only what the task requires.
- Preserve existing functionality unless a change explicitly requires otherwise.
- Do not rename files or reorganise directories without a clear reason.
- Do not introduce generic frameworks, state machines or plugin systems without approval.
- Reuse existing code where appropriate.
- Ask before making significant architectural changes.

## 14. Documenting Architectural Changes

Create or update an ADR in `docs/DECISIONS.md` when:

- Changing an established technology.
- Changing database entities or foreign-key policies.
- Changing Tauri command signatures or event contracts.
- Introducing a significant new abstraction.
- Changing file-access strategies.
- Changing the backup format.
- Changing annotation storage or anchoring.
- Changing security or privacy architecture.

Update all affected documentation.

## 15. Agent Workflow Requirements

Before making changes:

1. Read this file.
2. Read relevant documentation in `docs/`.
3. Inspect the existing implementation.
4. Confirm the requested task and its scope.
5. Identify any unresolved decisions before making assumptions.

While implementing:

1. Make the smallest correct change.
2. Follow existing architectural and coding conventions.
3. Add or update relevant tests.
4. Preserve unrelated functionality.
5. Update affected documentation.

After implementing:

1. Summarise the files changed.
2. Report tests and checks actually run.
3. Identify known limitations or unresolved issues.
4. Explain any decisions that require user approval.

Never invent requirements, silently change architectural decisions, or claim functionality is implemented when it is not.