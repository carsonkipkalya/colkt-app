# Project Structure

This document defines the recommended organisation of the Colkt codebase. It establishes where frontend features, backend domains, platform integrations, database migrations, and tests belong.

The structure should remain understandable and avoid unnecessary nesting or abstraction.

## 1. Project Root

```text
colkt-app/
├── AGENTS.md
├── README.md
├── LICENSE
├── package.json
├── package-lock.json
├── tsconfig.json
├── tsconfig.app.json
├── tsconfig.node.json
├── vite.config.ts
├── eslint.config.js
├── index.html
├── public/
├── src/
├── src-tauri/
├── docs/
└── tests/
```

Some files and directories are created by the Tauri and Vite scaffolding tools. Keep their generated configuration unless a documented project requirement calls for a change.

## 2. Frontend Structure

The React application lives in `src/`.

```text
src/
├── main.tsx
├── app/
├── components/
├── features/
│   ├── library/
│   ├── crates/
│   ├── tracking/
│   ├── resources/
│   ├── matching/
│   ├── player/
│   ├── reader/
│   │   └── engines/
│   ├── search/
│   ├── metadata/
│   ├── backup/
│   └── settings/
├── layouts/
├── hooks/
├── services/
├── styles/
└── types/
```

### Directory responsibilities

- `main.tsx` — React application entry point.
- `app/` — application root, routing, application-level state, and initialisation.
- `components/` — shared presentational components used by multiple features.
- `features/` — feature-specific components, hooks, and logic.
- `layouts/` — desktop and mobile application shells and responsive layout structures.
- `hooks/` — genuinely shared React hooks.
- `services/` — typed wrappers around Tauri commands and event subscriptions.
- `styles/` — global styles, CSS variables, design tokens, and shared styling.
- `types/` — shared TypeScript types, particularly IPC contracts.

### Feature organisation

Each feature should own its implementation rather than placing all application logic in shared directories.

For example:

```text
src/features/library/
├── components/
├── hooks/
├── types.ts
└── index.ts
```

This is an illustrative pattern, not a requirement to create every file or subdirectory immediately.

Feature-specific code should stay inside its feature directory. Move code into shared directories only when it is genuinely reused.

### Reader engines

`src/features/reader/engines/` contains the frontend adapters for the selected reader engines.

- EPUB: Foliate.js (provisional).
- PDF: Mozilla PDF.js (provisional).

Engine-specific types and behaviour must remain behind the reader integration boundary. They must not become application-wide data contracts.

## 3. Rust Backend Structure

The native backend lives in `src-tauri/`.

```text
src-tauri/
├── Cargo.toml
├── tauri.conf.json
├── build.rs
├── capabilities/
│   └── default.json
├── icons/
├── migrations/
│   └── 0001_initial.sql
└── src/
    ├── main.rs
    ├── lib.rs
    ├── commands/
    ├── domain/
    ├── db/
    ├── platform/
    │   ├── mod.rs
    │   ├── linux/
    │   └── android/
    ├── media/
    ├── reader/
    └── error.rs
```

### Directory responsibilities

- `main.rs` — native application entry point.
- `lib.rs` — Tauri builder, application state, command registration, and event setup.
- `commands/` — Tauri command handlers organised by domain, matching `docs/API.md`.
- `domain/` — business rules and domain operations.
- `db/` — SQLite connections, migrations, models, and queries.
- `platform/` — platform abstraction traits and their Linux and Android implementations.
- `media/` — `MediaEngine` abstraction and video engine implementations.
- `reader/` — reader-related backend interfaces, validation, and shared helpers.
- `error.rs` — shared application error type (`AppError`).

### Command organisation

Commands should be grouped by domain rather than placed in one large module.

For example:

```text
src-tauri/src/commands/
├── mod.rs
├── profile.rs
├── library.rs
├── tracking.rs
├── crates.rs
├── resources.rs
├── playback.rs
├── reader.rs
├── annotations.rs
├── metadata.rs
├── search.rs
├── backup.rs
└── settings.rs
```

The final module names should follow the actual command contracts in `docs/API.md`.

### Platform organisation

Platform-specific implementation belongs under:

```text
src-tauri/src/platform/
├── mod.rs
├── linux/
└── android/
```

Shared domain code should depend on platform interfaces rather than directly importing Linux or Android implementation types.

### Database migrations

SQL migrations belong in:

```text
src-tauri/migrations/
```

Use numbered, descriptive filenames:

```text
0001_initial.sql
0002_add_media_indexes.sql
0003_update_annotations.sql
```

Migrations are append-only. Never modify a migration that has already been applied. Every schema change requires a new migration and corresponding documentation updates.

## 4. Documentation Structure

Project documentation lives in `docs/`.

```text
docs/
├── ARCHITECTURE.md
├── DATABASE.md
├── FEATURES.md
├── API.md
├── PLATFORM.md
├── MEDIA.md
├── SECURITY.md
├── ROADMAP.md
├── DECISIONS.md
└── PROJECT_STRUCTURE.md
```

Each document has a distinct responsibility:

- `ARCHITECTURE.md` — system architecture and component boundaries.
- `DATABASE.md` — SQLite schema, relationships, migrations, and constraints.
- `FEATURES.md` — V1 features and acceptance criteria.
- `API.md` — frontend/backend commands, payloads, and events.
- `PLATFORM.md` — Linux and Android platform behaviour.
- `MEDIA.md` — video playback and book reader integration.
- `SECURITY.md` — security and privacy requirements.
- `ROADMAP.md` — implementation phases and release milestones.
- `DECISIONS.md` — accepted decisions, provisional choices, and pending prototypes.
- `PROJECT_STRUCTURE.md` — codebase organisation and directory responsibilities.

`AGENTS.md` remains at the project root and provides instructions for AI coding agents.

## 5. Test Structure

Cross-boundary integration tests belong in the root `tests/` directory.

```text
tests/
└── integration/
```

Tests closely tied to a Rust module may also live alongside that module in Rust's conventional test structure.

Frontend unit tests should generally live alongside the feature or module they test.

Examples:

```text
src/features/library/
├── components/
├── library.test.ts
└── types.ts
```

```text
src-tauri/src/domain/
└── tracking/
    └── tests.rs
```

Use the testing requirements in `AGENTS.md` and the relevant feature documentation.

## 6. Organisation Rules

1. Keep frontend and backend responsibilities separate.
2. Keep business rules in Rust, not React components.
3. Keep SQLite access exclusively in the Rust backend.
4. Organise frontend code by feature.
5. Organise backend commands by domain.
6. Keep Linux and Android implementations isolated behind shared interfaces.
7. Keep media and reader engine-specific details behind their respective abstractions.
8. Keep database migrations in `src-tauri/migrations/`.
9. Keep shared components genuinely reusable; do not move feature-specific code into shared directories prematurely.
10. Avoid unnecessary nesting and oversized modules.
11. Do not create empty directories or placeholder abstractions merely to match this diagram.
12. Do not restructure the project without a concrete requirement and, where appropriate, an approved ADR.

## 7. Implementation Guidance

The directory structure is a guide for organising implementation, not a requirement to build every directory before development begins.

Create directories and modules when their corresponding features are implemented.

When adding a new feature:

1. Identify whether it belongs to an existing domain or requires a new one.
2. Place frontend implementation in the appropriate feature directory.
3. Place business logic in the corresponding Rust domain module.
4. Add or update Tauri commands in the appropriate command module.
5. Update IPC types and contracts.
6. Add tests.
7. Update affected documentation.

The actual project structure must remain consistent with `AGENTS.md`, `ARCHITECTURE.md`, and the other project documentation.