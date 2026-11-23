pub mod migrations;

#[cfg(test)]
mod tests;

use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use rusqlite::Connection;

use crate::error::{AppError, ErrorCode};

/// Managed handle to the application's SQLite database.
///
/// The connection is shared with Tauri commands through application state and
/// serialised with a mutex, since `rusqlite::Connection` is not `Sync`.
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Opens the database at `path`, creating it and its parent directory if
    /// necessary, applies connection pragmas and pending migrations.
    pub fn open(path: &Path) -> Result<Self, AppError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        Self::configure(&conn)?;
        migrations::migrate(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn configure(conn: &Connection) -> Result<(), AppError> {
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA busy_timeout = 5000;",
        )?;
        Ok(())
    }

    /// Locks the shared connection for use by a command or service.
    ///
    /// Returns an error if the mutex is poisoned: a panic while a connection
    /// was in use may have left it in an uncertain state, so database
    /// operations must not silently continue on it.
    pub fn connection(&self) -> Result<MutexGuard<'_, Connection>, AppError> {
        self.conn
            .lock()
            .map_err(|_| AppError::new(ErrorCode::Internal, "database connection unavailable"))
    }
}
