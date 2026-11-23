use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, params};

use super::Database;
use super::migrations::{self, Migration};
use crate::error::ErrorCode;

const EXPECTED_TABLES: &[&str] = &[
    "schema_migrations",
    "profile",
    "app_security",
    "media_item",
    "movie",
    "tv_show",
    "season",
    "episode",
    "book_work",
    "book_edition",
    "external_identifier",
    "local_resource",
    "media_resource_link",
    "crate",
    "crate_membership",
    "movie_tracking",
    "show_tracking",
    "episode_tracking",
    "book_tracking",
    "annotation",
    "watched_folder",
    "watched_folder_resource",
    "metadata_cache",
    "app_setting",
    "media_item_fts",
];

static COUNTER: AtomicU64 = AtomicU64::new(0);

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Self {
        let unique = format!(
            "colkt-test-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
        let path = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn table_exists(conn: &Connection, name: &str) -> bool {
    conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
        params![name],
        |row| row.get::<_, i64>(0),
    )
    .unwrap()
        > 0
}

fn migration_versions(conn: &Connection) -> Vec<String> {
    let mut stmt = conn
        .prepare("SELECT version FROM schema_migrations ORDER BY version")
        .unwrap();
    stmt.query_map([], |row| row.get::<_, String>(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap()
}

#[test]
fn open_creates_database_and_applies_initial_migration() {
    let dir = TempDir::new();
    let db = Database::open(&dir.0.join("colkt.db")).unwrap();
    let conn = db.connection().unwrap();

    assert_eq!(migration_versions(&conn), ["0001_initial"]);
    for table in EXPECTED_TABLES {
        assert!(table_exists(&conn, table), "missing table {table}");
    }
    let applied_at: String = conn
        .query_row(
            "SELECT applied_at FROM schema_migrations WHERE version = '0001_initial'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert!(
        applied_at.ends_with('Z'),
        "applied_at not UTC: {applied_at}"
    );
}

#[test]
fn reopen_does_not_rerun_migrations() {
    let dir = TempDir::new();
    let path = dir.0.join("colkt.db");

    let applied_at = {
        let db = Database::open(&path).unwrap();
        let conn = db.connection().unwrap();
        conn.query_row(
            "SELECT applied_at FROM schema_migrations WHERE version = '0001_initial'",
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap()
    };

    let db = Database::open(&path).unwrap();
    let conn = db.connection().unwrap();
    assert_eq!(migration_versions(&conn), ["0001_initial"]);
    let applied_at_after: String = conn
        .query_row(
            "SELECT applied_at FROM schema_migrations WHERE version = '0001_initial'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(applied_at, applied_at_after);
}

#[test]
fn failed_migration_rolls_back_completely() {
    let conn = Connection::open_in_memory().unwrap();
    let bad = [Migration {
        version: "9999_bad",
        sql: "CREATE TABLE rollback_probe (id INTEGER);
              INSERT INTO missing_table VALUES (1);",
    }];

    let err = migrations::migrate_with(&conn, &bad).unwrap_err();
    assert_eq!(err.code, ErrorCode::Internal);
    assert!(!table_exists(&conn, "rollback_probe"));
    assert!(!table_exists(&conn, "schema_migrations"));
}

#[test]
fn unsupported_database_version_is_rejected() {
    let dir = TempDir::new();
    let path = dir.0.join("colkt.db");

    {
        let db = Database::open(&path).unwrap();
        db.connection()
            .unwrap()
            .execute(
                "INSERT INTO schema_migrations (version, applied_at)
                 VALUES ('9999_future', '2026-01-01T00:00:00.000Z')",
                [],
            )
            .unwrap();
    }

    let err = Database::open(&path).err().unwrap();
    assert_eq!(err.code, ErrorCode::Unsupported);
}

#[test]
fn fts5_is_available_and_triggers_maintain_index() {
    let dir = TempDir::new();
    let db = Database::open(&dir.0.join("colkt.db")).unwrap();
    let conn = db.connection().unwrap();

    conn.execute(
        "INSERT INTO media_item (id, media_type, title, added_at, updated_at)
         VALUES ('item-1', 'movie', 'The Big Lebowski',
                 '2026-01-01T00:00:00.000Z', '2026-01-01T00:00:00.000Z')",
        [],
    )
    .unwrap();

    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM media_item_fts WHERE media_item_fts MATCH 'Lebowski'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 1);

    conn.execute(
        "UPDATE media_item SET title = 'Fargo' WHERE id = 'item-1'",
        [],
    )
    .unwrap();

    let old_hits: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM media_item_fts WHERE media_item_fts MATCH 'Lebowski'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let new_hits: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM media_item_fts WHERE media_item_fts MATCH 'Fargo'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(old_hits, 0);
    assert_eq!(new_hits, 1);
}

#[test]
fn schema_constraints_are_enforced() {
    let dir = TempDir::new();
    let db = Database::open(&dir.0.join("colkt.db")).unwrap();
    let conn = db.connection().unwrap();

    let bad_type = conn.execute(
        "INSERT INTO media_item (id, media_type, title, added_at, updated_at)
         VALUES ('bad-1', 'game', 'Not Media',
                 '2026-01-01T00:00:00.000Z', '2026-01-01T00:00:00.000Z')",
        [],
    );
    assert!(bad_type.is_err());

    conn.execute(
        "INSERT INTO media_item (id, media_type, title, added_at, updated_at)
         VALUES ('movie-1', 'movie', 'Valid Movie',
                 '2026-01-01T00:00:00.000Z', '2026-01-01T00:00:00.000Z')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO movie (media_item_id, runtime_seconds)
         VALUES ('movie-1', 5400)",
        [],
    )
    .unwrap();

    let bad_rating = conn.execute(
        "INSERT INTO movie_tracking
             (media_item_id, status, playback_seconds, rating_10, updated_at)
         VALUES ('movie-1', 'watching', 0, 11, '2026-01-01T00:00:00.000Z')",
        [],
    );
    assert!(bad_rating.is_err());

    let orphan_tracking = conn.execute(
        "INSERT INTO movie_tracking
             (media_item_id, status, playback_seconds, rating_10, updated_at)
         VALUES ('missing-item', 'watching', 0, NULL, '2026-01-01T00:00:00.000Z')",
        [],
    );
    assert!(orphan_tracking.is_err());
}

#[test]
fn valid_ordered_migration_list_applies_in_order() {
    let conn = Connection::open_in_memory().unwrap();
    let migrations = [
        Migration {
            version: "0001_first",
            sql: "CREATE TABLE probe_a (id INTEGER);",
        },
        Migration {
            version: "0002_second",
            sql: "CREATE TABLE probe_b (id INTEGER);",
        },
    ];

    migrations::migrate_with(&conn, &migrations).unwrap();

    assert!(table_exists(&conn, "probe_a"));
    assert!(table_exists(&conn, "probe_b"));
    assert_eq!(migration_versions(&conn), ["0001_first", "0002_second"]);
}

#[test]
fn duplicate_migration_versions_are_rejected_without_changes() {
    let conn = Connection::open_in_memory().unwrap();
    let migrations = [
        Migration {
            version: "0001_dup",
            sql: "CREATE TABLE should_not_exist (id INTEGER);",
        },
        Migration {
            version: "0001_dup",
            sql: "CREATE TABLE also_should_not_exist (id INTEGER);",
        },
    ];

    let err = migrations::migrate_with(&conn, &migrations).err().unwrap();
    assert_eq!(err.code, ErrorCode::Internal);
    assert!(!table_exists(&conn, "should_not_exist"));
    assert!(!table_exists(&conn, "also_should_not_exist"));
    assert!(!table_exists(&conn, "schema_migrations"));
}

#[test]
fn out_of_order_migration_versions_are_rejected_without_changes() {
    let conn = Connection::open_in_memory().unwrap();
    let migrations = [
        Migration {
            version: "0002_second",
            sql: "CREATE TABLE should_not_exist (id INTEGER);",
        },
        Migration {
            version: "0001_first",
            sql: "CREATE TABLE also_should_not_exist (id INTEGER);",
        },
    ];

    let err = migrations::migrate_with(&conn, &migrations).err().unwrap();
    assert_eq!(err.code, ErrorCode::Internal);
    assert!(!table_exists(&conn, "should_not_exist"));
    assert!(!table_exists(&conn, "also_should_not_exist"));
    assert!(!table_exists(&conn, "schema_migrations"));
}

#[test]
fn poisoned_mutex_returns_structured_error() {
    let dir = TempDir::new();
    let db = Database::open(&dir.0.join("colkt.db")).unwrap();

    let poisoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = db.connection().unwrap();
        panic!("simulated panic while holding the connection");
    }));
    assert!(poisoned.is_err());

    let err = db.connection().err().unwrap();
    assert_eq!(err.code, ErrorCode::Internal);
    assert_eq!(err.details, None);
}
