# Security and Privacy

## 1. Security Principles

Colkt is a private, offline-first personal media library. Security must protect the user's application data, preserve control over their media files, and minimise unnecessary access to the device.

Core principles:

1. Local-first: library data is stored locally.
2. No account requirement: V1 has no online accounts or authentication server.
3. Least privilege: request only the filesystem and platform permissions required.
4. User control: never modify or delete original media files.
5. Secure defaults: validate inputs and restrict access to sensitive operations.
6. Offline operation: core library functionality must not depend on network availability.
7. Transparent limitations: clearly communicate what application security features do and do not protect.

## 2. Application Passcode

### 2.1 Purpose

The optional application passcode prevents casual access to Colkt when someone opens or resumes the application.

It is an application access gate, not full-device security or database encryption.

### 2.2 Storage

- Store only a password hash, never the plaintext passcode.
- Use Argon2id or another appropriately configured modern password hashing algorithm.
- Generate a unique cryptographically secure random salt.
- Store the algorithm parameters and salt with the encoded hash.
- Never log the passcode or hash.
- Never return the hash through a frontend command or event.
- Keep verification inside the Rust backend.

### 2.3 Verification

The backend is authoritative for passcode verification.

- Verify passcodes using constant-time-safe verification provided by the password hashing library.
- Track failed attempts.
- Apply temporary lockout after repeated failures.
- Enforce lockout in the backend, not in React.
- Return only the verification result and lockout information required by the frontend.
- Do not permanently lock the user out because of failed attempts.

### 2.4 Locking Behaviour

When passcode protection is enabled:

- Require the passcode on cold launch.
- Require the passcode when returning from the background on Android.
- Lock when the user explicitly locks the application.
- Clear sensitive in-memory application state when locking where practical.
- Do not assume the application receives a shutdown callback.
- Persist important library and playback state independently of the lock screen.

The exact Linux desktop background and inactivity-locking behaviour must be defined during implementation and documented consistently across platforms.

### 2.5 Important Limitation

The passcode does not encrypt:

- The SQLite database.
- Original media files.
- Backup files.
- Application configuration files.
- Other files stored outside the application's protected interface.

Anyone with sufficient access to the device or application data directory may be able to inspect local data.

The application must communicate this limitation when users enable passcode protection.

## 3. Database Security

### 3.1 SQLite Access

- SQLite is accessed only through the Rust backend.
- Frontend components must never open or query the database directly.
- All database queries use parameterised statements.
- User-controlled values must never be interpolated into SQL statements.
- Database migrations must be versioned and reviewed.
- Foreign-key constraints must be enabled.
- Database operations must use transactions where multiple related records are modified.

### 3.2 Data Directory

- Store application data in the platform's appropriate application-data directory.
- Do not store the database in the project directory or a publicly accessible media directory.
- Use restrictive filesystem permissions where supported.
- Avoid exposing database paths through frontend interfaces.
- Do not include database files in logs or diagnostic reports.

### 3.3 Database Integrity

- Validate records before inserting or updating them.
- Use database constraints for identifiers, relationships, and numeric ranges.
- Handle database errors without exposing internal SQL details to the frontend.
- Protect migrations against partial execution.
- Use SQLite's supported backup mechanisms or an equivalent consistent snapshot procedure.
- Account for WAL and journal files during backup and restore.

### 3.4 Encryption

Database encryption is not included in V1.

Do not describe the passcode as encryption or imply that local database contents are protected against someone who can access the device's application-data directory.

Database encryption may be considered in a future version through a separately reviewed design.

## 4. Filesystem and Media Security

### 4.1 Original Media Ownership

User media files remain under the user's control.

Colkt must never automatically:

- Copy original media files into its own storage.
- Move files.
- Rename files.
- Modify file contents.
- Delete original files.

Reading, playback, scanning, and metadata extraction must not alter original media files.

### 4.2 Resource Access

- Access files only after the user selects them or grants access through an appropriate platform mechanism.
- Keep filesystem operations in the Rust backend and platform adapters.
- Validate that a resource belongs to the expected application operation or session.
- Treat inaccessible or revoked resources as unavailable.
- Never assume that a stored path or URI guarantees current access.
- Release temporary handles when operations finish.

### 4.3 Linux

- Respect Flatpak sandbox restrictions.
- Use portal-backed file and folder access.
- Avoid unrestricted filesystem permissions.
- Do not expose arbitrary host filesystem access through frontend commands.
- Validate paths and resource handles at the backend boundary.

### 4.4 Android

- Use Storage Access Framework (SAF).
- Persist URI permissions only when supported and required.
- Verify access before reading a resource.
- Do not convert content URIs into assumed filesystem paths.
- Handle revoked permissions gracefully.
- Do not request broad storage permissions when SAF provides the required access.

### 4.5 Path and Resource Validation

- Reject invalid resource identifiers.
- Prevent path traversal when resolving filesystem-backed resources.
- Do not trust filenames or extensions as proof of file content.
- Validate file formats before passing resources to media or reader engines.
- Prevent resource identifiers supplied by the frontend from bypassing access checks.
- Restrict temporary resource URLs to their intended session and lifetime.

## 5. Tauri and Frontend Security

### 5.1 Command Boundaries

All Tauri commands are security boundaries.

- Validate every command input in Rust.
- Verify that referenced database records exist.
- Check ownership and relationships between records.
- Reject invalid state transitions.
- Avoid exposing internal filesystem, database, or platform operations unnecessarily.
- Return safe error messages.

Frontend validation is for usability only. It does not replace backend validation.

### 5.2 Tauri Capabilities

Use Tauri's capability and permission system with least privilege.

- Grant only the APIs required by the application.
- Scope permissions to the application windows that need them.
- Avoid unrestricted shell, filesystem, process, or HTTP permissions.
- Do not grant capabilities simply because a plugin makes them available.
- Review capability changes whenever a plugin or native integration is added.

### 5.3 Content Security Policy

Production builds must define a restrictive Content Security Policy (CSP).

- Avoid unrestricted `unsafe-eval`.
- Avoid broad wildcard sources.
- Restrict script and style sources to what the application requires.
- Restrict network connections to required metadata providers and resource mechanisms.
- Allow only the controlled local resource-loading mechanism needed by playback and reading.
- Review CSP changes alongside reader and media-engine integration.

The development server may require additional development-only allowances. These must not leak into production configuration.

### 5.4 Frontend State

- Do not store passcodes in local storage, session storage, or persistent frontend state.
- Do not expose password hashes to React.
- Avoid retaining sensitive data longer than necessary.
- Clear sensitive UI state when the application locks.
- Treat IPC responses as runtime data and handle errors safely.
- Do not assume that hiding a UI element provides backend access control.

## 6. Network and Metadata Providers

Colkt is offline-first, but optional metadata retrieval uses external services.

### 6.1 Network Rules

- No network request is required for normal local library access.
- Metadata retrieval must be initiated by an appropriate user action or documented application behaviour.
- Network failures must not prevent local library access.
- Use HTTPS for external provider requests.
- Do not send local media files to metadata providers.
- Send only the information necessary for a metadata lookup.
- Do not transmit library contents, tracking history, annotations, or personal profile information as part of ordinary metadata requests.

### 6.2 Provider Integration

Metadata providers include:

- TMDB.
- TVmaze.
- Open Library.

Requirements:

- Keep provider integrations behind Rust backend adapters.
- Store provider credentials or API configuration outside frontend source code.
- Respect provider terms, rate limits, and attribution requirements.
- Validate provider responses before storing them.
- Treat returned metadata and artwork URLs as untrusted external data.
- Do not automatically merge records based on provider identifiers or fuzzy matching.

### 6.3 Network Security

- Validate external URLs before requesting resources.
- Restrict requests to supported provider domains where practical.
- Avoid server-side request forgery through arbitrary user-supplied URLs.
- Set reasonable connection and response-size limits.
- Handle malformed responses and timeouts safely.
- Do not execute scripts or active content returned by metadata providers.

## 7. Media and Document Security

### 7.1 Untrusted Files

Local media files, EPUB documents, PDF documents, subtitles, and metadata are untrusted input.

- Do not assume a locally selected file is safe.
- Use maintained media and document libraries.
- Keep media-engine integrations isolated.
- Handle parser and decoder failures without crashing the application.
- Avoid loading arbitrary scripts or executable content from documents.
- Do not grant document content unrestricted access to application APIs.

### 7.2 EPUB and PDF

- Use controlled resource URLs for reader access.
- Do not expose unrestricted filesystem APIs to reader content.
- Apply the WebView's security restrictions to document rendering.
- Treat embedded links, scripts, images, and other document content as untrusted.
- Do not allow document content to invoke arbitrary Tauri commands.
- Keep annotations in the application database rather than modifying source documents.

### 7.3 Subtitles

- Treat external subtitle files as untrusted text input.
- Do not execute subtitle content.
- Escape text appropriately before rendering.
- Handle malformed encoding and oversized files safely.

### 7.4 Media Engine Isolation

- Keep engine-specific code behind the shared media interface.
- Do not expose engine handles or internal objects to React.
- Validate commands and resource access before passing them to engines.
- Handle malformed media and engine errors without panics.
- Ensure session cleanup releases associated resources.

## 8. Backup Security

### 8.1 Backup Contents

Backups contain private application data, including:

- Library entries.
- Tracking and ratings.
- Crates.
- Annotations.
- Profile information.
- Settings.
- Local resource references.
- External identifiers.

Original media files are excluded by default.

### 8.2 Backup Handling

- Treat backup files as sensitive user data.
- Validate backup format and schema version before restoring.
- Reject malformed or incompatible backups safely.
- Prevent archive path traversal during extraction.
- Limit archive size and decompression expansion.
- Do not overwrite arbitrary filesystem locations during restore.
- Perform restore operations transactionally.
- Require explicit confirmation before replace-mode restore.
- Preserve unavailable resource references without deleting related library data.

### 8.3 Encryption

Backup encryption is not included in V1 unless separately approved.

Users must be informed that exported backups may contain readable private library information and should be stored securely.

## 9. Logging and Diagnostics

- Never log passcodes or password hashes.
- Avoid logging personal annotations or reading notes.
- Avoid logging complete library records.
- Avoid logging unnecessary absolute filesystem paths or content URIs.
- Do not include media contents in diagnostic output.
- Redact sensitive data from errors and diagnostics.
- Keep logs useful for troubleshooting without collecting unnecessary personal information.

## 10. Dependency Security

- Prefer maintained dependencies with active security support.
- Avoid unnecessary dependencies.
- Review dependency permissions and capabilities.
- Keep Rust and JavaScript dependencies updated.
- Monitor security advisories for Tauri, Rust crates, WebView components, GStreamer, Media3, Foliate.js, and PDF.js.
- Remove unused dependencies.
- Review native plugins and third-party integrations before adding them.
- Do not treat a successful build as proof that dependencies are secure.

## 11. Threat Model and Limitations

### 11.1 In-Scope Threats

V1 should mitigate:

- Casual unauthorised access to an unlocked or unattended application.
- Accidental modification or deletion of original media.
- Malformed user input reaching database operations.
- Unauthorised frontend access to backend resources.
- Malformed or malicious media and document content exploiting application integrations.
- Accidental exposure of private data through logs.
- Corruption caused by incomplete backup or restore operations.
- Unnecessary filesystem and network permissions.

### 11.2 Out-of-Scope Protections

V1 does not guarantee protection against:

- A person with administrator or root access to the device.
- Malware already running with the user's permissions.
- Physical access to an unencrypted database or backup.
- Forensic recovery of deleted application data.
- Device compromise or operating-system vulnerabilities.
- Someone who can bypass the application and directly inspect its files.

Users should rely on operating-system account security and full-disk encryption for stronger device-level protection.

## 12. Security Validation Checklist

### Passcode

- [ ] Passcode is never stored in plaintext.
- [ ] Password hashing uses a modern KDF.
- [ ] Failed attempts trigger backend-enforced temporary lockout.
- [ ] Passcode is never exposed through IPC responses.
- [ ] Passcode is required on cold launch when enabled.
- [ ] Android resume locking works as specified.
- [ ] Sensitive UI state is cleared when locking.

### Database

- [ ] All database queries are parameterised.
- [ ] Foreign-key enforcement is enabled.
- [ ] Backend validates all mutations.
- [ ] Migrations are versioned and transactional.
- [ ] Database files remain in the application-data directory.
- [ ] Backup and restore handle SQLite WAL correctly.

### Filesystem

- [ ] Linux file access uses appropriate portals.
- [ ] Android file access uses SAF.
- [ ] Revoked permissions are handled safely.
- [ ] Path traversal is prevented.
- [ ] Original media files remain unchanged.
- [ ] Resource URLs are session-scoped.

### Tauri and Frontend

- [ ] Production CSP is configured.
- [ ] Tauri capabilities follow least privilege.
- [ ] Frontend cannot access SQLite directly.
- [ ] Sensitive values are not stored in browser storage.
- [ ] Backend validates all IPC inputs.
- [ ] Document content cannot invoke arbitrary application commands.

### Network and Dependencies

- [ ] External provider requests use HTTPS.
- [ ] Local media files are never uploaded to metadata providers.
- [ ] Provider responses are validated.
- [ ] Network failures do not break offline functionality.
- [ ] Dependencies are reviewed and maintained.

### Backup and Logging

- [ ] Backup files are treated as sensitive.
- [ ] Restore validates archives before extraction.
- [ ] Archive path traversal is prevented.
- [ ] Restore is transactional.
- [ ] Logs do not expose passcodes or unnecessary personal data.

## 13. Security Change Management

Security-sensitive changes require review before implementation.

Examples include:

- New Tauri plugins or capabilities.
- Changes to passcode storage or verification.
- New filesystem access mechanisms.
- New network providers.
- Changes to reader or media-engine integration.
- Backup format or restore changes.
- New handling of private user data.

Document significant architectural security decisions in `docs/DECISIONS.md`.

Security requirements in this document take precedence over convenience when the two conflict.