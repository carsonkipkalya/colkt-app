use std::collections::HashSet;

use rusqlite::Connection;

use crate::error::{AppError, ErrorCode};

/// A versioned SQL migration. `sql` is kept in `src-tauri/migrations/` as a
/// plain `.sql` file; this list only binds versions to their contents.
pub struct Migration {
    pub version: &'static str,
    pub sql: &'static str,
}

/// All known migrations, in application order.
pub const MIGRATIONS: &[Migration] = &[Migration {
    version: "0001_initial",
    sql: include_str!("../../migrations/0001_initial.sql"),
}];

/// Applies all pending migrations to `conn`.
pub fn migrate(conn: &Connection) -> Result<(), AppError> {
    migrate_with(conn, MIGRATIONS)
}

/// Applies pending migrations from `migrations`. Each migration runs in its
/// own transaction together with its `schema_migrations` bookkeeping row, so
/// a failure leaves neither partial schema changes nor a recorded version.
pub(crate) fn migrate_with(conn: &Connection, migrations: &[Migration]) -> Result<(), AppError> {
    validate_migrations(migrations)?;

    let has_table = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master
         WHERE type = 'table' AND name = 'schema_migrations'",
        [],
        |row| row.get::<_, i64>(0),
    )? > 0;

    let applied: HashSet<String> = if has_table {
        let mut stmt = conn.prepare("SELECT version FROM schema_migrations")?;
        stmt.query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<_, _>>()?
    } else {
        HashSet::new()
    };

    let known: HashSet<&str> = migrations.iter().map(|m| m.version).collect();
    for version in &applied {
        if !known.contains(version.as_str()) {
            return Err(AppError::new(
                ErrorCode::Unsupported,
                format!("database schema version '{version}' is not supported by this application"),
            ));
        }
    }

    for migration in migrations {
        if applied.contains(migration.version) {
            continue;
        }
        let tx = conn.unchecked_transaction()?;
        tx.execute_batch(migration.sql)?;
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version TEXT PRIMARY KEY,
                applied_at TEXT NOT NULL
            );",
        )?;
        tx.execute(
            "INSERT INTO schema_migrations (version, applied_at)
             VALUES (?1, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))",
            [migration.version],
        )?;
        tx.commit()?;
    }

    Ok(())
}

/// Validates migration definitions before any database access. Versions must
/// be unique and in strictly increasing order so application order is
/// unambiguous and duplicates cannot be silently skipped.
fn validate_migrations(migrations: &[Migration]) -> Result<(), AppError> {
    for pair in migrations.windows(2) {
        let (previous, next) = (&pair[0], &pair[1]);
        if previous.version == next.version {
            return Err(AppError::new(
                ErrorCode::Internal,
                format!("duplicate migration version '{}'", previous.version),
            ));
        }
        if previous.version > next.version {
            return Err(AppError::new(
                ErrorCode::Internal,
                format!(
                    "migration versions out of order: '{}' must precede '{}'",
                    previous.version, next.version
                ),
            ));
        }
    }
    Ok(())
}
