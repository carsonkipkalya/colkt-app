# Colkt

**A private, offline-first personal media library for the things you watch and read.**

Colkt is a personal media library for organising movies, TV shows, and books in one place. It brings together your library, collections (crates), viewing and reading progress, and locally stored media without taking ownership of your files.

Colkt is designed for Linux desktop and Android, with privacy, local control, and offline use at its core.

## Project status

Colkt is currently in early development. The application foundation and technology stack are being established before the main features are implemented.

Features, architecture, database design, API contracts, platform requirements, and development phases are documented in the `docs/` directory.

Interfaces, integrations, and platform-specific media engines may change as implementation prototypes are tested.

## Core principles

- **Private by design:** Your library and personal tracking data stay on your device.
- **Offline-first:** Core library functionality does not depend on an internet connection.
- **Your files remain yours:** Colkt does not copy, move, rename, modify, or delete your original media files.
- **One unified library:** Organise movies, TV shows, and books together.
- **User-controlled matching:** Metadata and local media matches are suggestions. You confirm them before they are linked.
- **No unnecessary accounts:** V1 does not require an online account, server, or cloud service.

## Planned features

- Manage movies, TV shows, and books in one library.
- Create custom collections called **crates**.
- Track watched movies, TV episodes, and reading progress.
- Discover and link locally stored media files.
- Identify media using metadata providers, with manual confirmation.
- Play supported video files inside the application.
- Read EPUB and PDF books.
- Add reading annotations and highlights.
- Search the library.
- Back up and restore library data.

See [`docs/FEATURES.md`](docs/FEATURES.md) for the feature scope and V1 limitations.

## Technology stack

| Area | Technology |
|---|---|
| Desktop and Android application | Tauri 2 |
| Frontend | React |
| Language | TypeScript |
| Frontend tooling | Vite |
| Backend | Rust |
| Database | SQLite |
| Styling | CSS, CSS Modules, and CSS variables |
| Linux distribution | Flatpak |
| Android distribution | APK |

Planned media and document engines:

- **Linux video:** GStreamer
- **Android video:** AndroidX Media3
- **EPUB reader:** Foliate.js
- **PDF reader:** Mozilla PDF.js

Media engines and reader integrations are subject to implementation and platform testing. See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md), [`docs/MEDIA.md`](docs/MEDIA.md), and [`docs/PLATFORM.md`](docs/PLATFORM.md).

## Development setup

### Requirements

- Linux development environment
- Node.js and npm
- Rust toolchain and Cargo
- Tauri 2 development dependencies
- Android SDK and NDK for Android development

The project is currently being developed on Fedora Linux.

### Install dependencies

Clone the repository and enter the project directory:

```bash
git clone <repository-url>
cd colkt-app
```

Install frontend dependencies:

```bash
npm install
```

### Run the development application

Start the Tauri development application:

```bash
cargo tauri dev
```

This starts the Vite development server and launches the Colkt desktop application.

### Build the frontend

```bash
npm run build
```

### Run linting

```bash
npm run lint
```

## Project structure

```text
colkt-app/
├── src/                  # React and TypeScript frontend
├── src-tauri/            # Rust backend and Tauri configuration
├── docs/                 # Project documentation
├── tests/                # Automated tests
├── package.json
├── vite.config.ts
└── README.md
```

See [`docs/PROJECT_STRUCTURE.md`](docs/PROJECT_STRUCTURE.md) for the planned organisation of the codebase.

## Documentation

| Document | Description |
|---|---|
| [`AGENTS.md`](AGENTS.md) | Instructions for AI coding agents |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Application architecture and technical boundaries |
| [`docs/DATABASE.md`](docs/DATABASE.md) | Database schema and data relationships |
| [`docs/FEATURES.md`](docs/FEATURES.md) | Feature definitions and V1 scope |
| [`docs/API.md`](docs/API.md) | Frontend and backend command contracts |
| [`docs/PLATFORM.md`](docs/PLATFORM.md) | Linux and Android platform requirements |
| [`docs/MEDIA.md`](docs/MEDIA.md) | Video playback, reading, and annotations |
| [`docs/SECURITY.md`](docs/SECURITY.md) | Security principles and requirements |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Development phases and milestones |
| [`docs/DECISIONS.md`](docs/DECISIONS.md) | Architecture decision records |
| [`docs/PROJECT_STRUCTURE.md`](docs/PROJECT_STRUCTURE.md) | Codebase organisation |

## V1 limitations

The initial release is focused on a single-device, local library.

V1 does not include:

- Cloud synchronisation.
- Online accounts or hosted services.
- Multiple profiles per installation.
- Remote media streaming.
- Social features or recommendations.
- Automatic merging of media matches.

These limitations may be reconsidered for future releases.

## License

Colkt is intended to be free and open-source software. The project's license will be specified in the repository's `LICENSE` file.