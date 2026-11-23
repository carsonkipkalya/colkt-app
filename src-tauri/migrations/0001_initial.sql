-- 0001_initial.sql — initial Colkt schema.
-- Source of truth: docs/DATABASE.md

CREATE TABLE schema_migrations (
    version TEXT PRIMARY KEY,
    applied_at TEXT NOT NULL
);

CREATE TABLE profile (
    id TEXT PRIMARY KEY CHECK (id = 'default'),
    name TEXT NOT NULL,
    picture_uri TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE app_security (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    passcode_hash TEXT,
    failed_attempts INTEGER NOT NULL DEFAULT 0
        CHECK (failed_attempts >= 0),
    locked_until TEXT,
    updated_at TEXT NOT NULL
);

CREATE TABLE media_item (
    id TEXT PRIMARY KEY,
    media_type TEXT NOT NULL
        CHECK (media_type IN ('movie', 'tv_show', 'book')),
    title TEXT NOT NULL,
    sort_title TEXT,
    description TEXT,
    release_year INTEGER,
    poster_uri TEXT,
    added_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE movie (
    media_item_id TEXT PRIMARY KEY
        REFERENCES media_item(id) ON DELETE CASCADE,
    runtime_seconds INTEGER
        CHECK (runtime_seconds IS NULL OR runtime_seconds >= 0)
);

CREATE TABLE tv_show (
    media_item_id TEXT PRIMARY KEY
        REFERENCES media_item(id) ON DELETE CASCADE
);

CREATE TABLE season (
    id TEXT PRIMARY KEY,
    tv_show_id TEXT NOT NULL
        REFERENCES tv_show(media_item_id) ON DELETE CASCADE,
    season_number INTEGER NOT NULL CHECK (season_number >= 0),
    title TEXT,
    description TEXT,
    UNIQUE (tv_show_id, season_number)
);

CREATE TABLE episode (
    id TEXT PRIMARY KEY,
    season_id TEXT NOT NULL
        REFERENCES season(id) ON DELETE CASCADE,
    episode_number INTEGER NOT NULL CHECK (episode_number > 0),
    title TEXT,
    description TEXT,
    runtime_seconds INTEGER
        CHECK (runtime_seconds IS NULL OR runtime_seconds >= 0),
    UNIQUE (season_id, episode_number)
);

CREATE TABLE book_work (
    media_item_id TEXT PRIMARY KEY
        REFERENCES media_item(id) ON DELETE CASCADE,
    original_title TEXT,
    first_published_year INTEGER
);

CREATE TABLE book_edition (
    id TEXT PRIMARY KEY,
    book_work_id TEXT NOT NULL
        REFERENCES book_work(media_item_id) ON DELETE CASCADE,
    title TEXT,
    isbn_13 TEXT,
    isbn_10 TEXT,
    publisher TEXT,
    publication_year INTEGER,
    language TEXT,
    page_count INTEGER
        CHECK (page_count IS NULL OR page_count > 0),
    format TEXT
        CHECK (format IN ('physical', 'ebook', 'pdf', 'other'))
);

CREATE TABLE external_identifier (
    id TEXT PRIMARY KEY,
    media_item_id TEXT NOT NULL
        REFERENCES media_item(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    external_id TEXT NOT NULL,
    UNIQUE (provider, external_id)
);

CREATE TABLE local_resource (
    id TEXT PRIMARY KEY,
    platform_kind TEXT NOT NULL
        CHECK (platform_kind IN ('file_path', 'content_uri')),
    identifier TEXT NOT NULL,
    display_name TEXT,
    size_bytes INTEGER
        CHECK (size_bytes IS NULL OR size_bytes >= 0),
    mime_hint TEXT,
    available INTEGER NOT NULL DEFAULT 1
        CHECK (available IN (0, 1)),
    last_seen_at TEXT,
    created_at TEXT NOT NULL,
    UNIQUE (platform_kind, identifier)
);

CREATE TABLE media_resource_link (
    id TEXT PRIMARY KEY,
    media_item_id TEXT NOT NULL
        REFERENCES media_item(id) ON DELETE CASCADE,
    local_resource_id TEXT NOT NULL
        REFERENCES local_resource(id) ON DELETE CASCADE,
    kind TEXT NOT NULL
        CHECK (kind IN ('primary', 'additional', 'subtitle')),
    subtitle_language TEXT,
    confirmed_at TEXT NOT NULL,
    UNIQUE (media_item_id, local_resource_id, kind)
);

CREATE TABLE crate (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    allowed_media_types TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE crate_membership (
    crate_id TEXT NOT NULL
        REFERENCES crate(id) ON DELETE CASCADE,
    media_item_id TEXT NOT NULL
        REFERENCES media_item(id) ON DELETE CASCADE,
    added_at TEXT NOT NULL,
    position INTEGER,
    PRIMARY KEY (crate_id, media_item_id)
);

CREATE TABLE movie_tracking (
    media_item_id TEXT PRIMARY KEY
        REFERENCES movie(media_item_id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'want_to_watch'
        CHECK (status IN ('want_to_watch', 'watching', 'completed')),
    playback_seconds REAL NOT NULL DEFAULT 0
        CHECK (playback_seconds >= 0),
    duration_seconds REAL
        CHECK (duration_seconds IS NULL OR duration_seconds >= 0),
    rating_10 INTEGER
        CHECK (rating_10 IS NULL OR rating_10 BETWEEN 1 AND 10),
    updated_at TEXT NOT NULL
);

CREATE TABLE show_tracking (
    media_item_id TEXT PRIMARY KEY
        REFERENCES tv_show(media_item_id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'want_to_watch'
        CHECK (status IN ('want_to_watch', 'watching', 'completed')),
    rating_10 INTEGER
        CHECK (rating_10 IS NULL OR rating_10 BETWEEN 1 AND 10),
    updated_at TEXT NOT NULL
);

CREATE TABLE episode_tracking (
    episode_id TEXT PRIMARY KEY
        REFERENCES episode(id) ON DELETE CASCADE,
    watched INTEGER NOT NULL DEFAULT 0
        CHECK (watched IN (0, 1)),
    watched_at TEXT,
    playback_seconds REAL NOT NULL DEFAULT 0
        CHECK (playback_seconds >= 0),
    duration_seconds REAL
        CHECK (duration_seconds IS NULL OR duration_seconds >= 0),
    updated_at TEXT NOT NULL
);

CREATE TABLE book_tracking (
    media_item_id TEXT PRIMARY KEY
        REFERENCES book_work(media_item_id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'want_to_read'
        CHECK (status IN ('want_to_read', 'reading', 'completed')),
    current_page INTEGER
        CHECK (current_page IS NULL OR current_page >= 0),
    progress_percent REAL
        CHECK (progress_percent IS NULL OR progress_percent BETWEEN 0 AND 100),
    rating_stars INTEGER
        CHECK (rating_stars IS NULL OR rating_stars BETWEEN 1 AND 5),
    format_kind TEXT
        CHECK (format_kind IN ('physical', 'digital')),
    preferred_edition_id TEXT
        REFERENCES book_edition(id) ON DELETE SET NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE annotation (
    id TEXT PRIMARY KEY,
    media_item_id TEXT NOT NULL
        REFERENCES book_work(media_item_id) ON DELETE CASCADE,
    book_edition_id TEXT
        REFERENCES book_edition(id) ON DELETE SET NULL,
    resource_id TEXT
        REFERENCES local_resource(id) ON DELETE SET NULL,
    kind TEXT NOT NULL
        CHECK (kind IN ('highlight', 'note')),
    format TEXT NOT NULL
        CHECK (format IN ('epub', 'pdf')),
    selector_json TEXT NOT NULL,
    quoted_text TEXT,
    note TEXT,
    color TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE watched_folder (
    id TEXT PRIMARY KEY,
    platform_kind TEXT NOT NULL
        CHECK (platform_kind IN ('file_path', 'document_tree')),
    identifier TEXT NOT NULL,
    display_name TEXT,
    recursive INTEGER NOT NULL DEFAULT 1
        CHECK (recursive IN (0, 1)),
    scan_on_startup INTEGER NOT NULL DEFAULT 1
        CHECK (scan_on_startup IN (0, 1)),
    last_scan_at TEXT,
    created_at TEXT NOT NULL,
    UNIQUE (platform_kind, identifier)
);

CREATE TABLE watched_folder_resource (
    watched_folder_id TEXT NOT NULL
        REFERENCES watched_folder(id) ON DELETE CASCADE,
    local_resource_id TEXT NOT NULL
        REFERENCES local_resource(id) ON DELETE CASCADE,
    discovered_at TEXT NOT NULL,
    PRIMARY KEY (watched_folder_id, local_resource_id)
);

CREATE TABLE metadata_cache (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL,
    cache_key TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    fetched_at TEXT NOT NULL,
    expires_at TEXT,
    UNIQUE (provider, cache_key)
);

CREATE TABLE app_setting (
    key TEXT PRIMARY KEY,
    value_json TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Indexes (docs/DATABASE.md). The UNIQUE constraints on
-- season(tv_show_id, season_number) and episode(season_id, episode_number)
-- already provide the equivalent indexes, so they are not duplicated here.

CREATE INDEX idx_media_item_media_type
    ON media_item(media_type);

CREATE INDEX idx_media_item_title
    ON media_item(title);

CREATE INDEX idx_media_resource_link_item
    ON media_resource_link(media_item_id);

CREATE INDEX idx_media_resource_link_resource
    ON media_resource_link(local_resource_id);

CREATE INDEX idx_crate_membership_item
    ON crate_membership(media_item_id);

CREATE INDEX idx_external_identifier_item
    ON external_identifier(media_item_id);

CREATE INDEX idx_episode_tracking_watched
    ON episode_tracking(watched);

CREATE INDEX idx_annotation_item
    ON annotation(media_item_id);

CREATE INDEX idx_annotation_resource
    ON annotation(resource_id);

CREATE INDEX idx_watched_folder_resource_resource
    ON watched_folder_resource(local_resource_id);

-- Full-text search over media_item title fields (docs/DATABASE.md).
-- media_item uses a TEXT primary key, so the FTS table uses the implicit
-- integer rowid as its content rowid via an external-content table.

CREATE VIRTUAL TABLE media_item_fts USING fts5(
    title,
    sort_title,
    description,
    content = 'media_item',
    content_rowid = 'rowid'
);

CREATE TRIGGER media_item_fts_insert
AFTER INSERT ON media_item BEGIN
    INSERT INTO media_item_fts(rowid, title, sort_title, description)
    VALUES (new.rowid, new.title, new.sort_title, new.description);
END;

CREATE TRIGGER media_item_fts_delete
AFTER DELETE ON media_item BEGIN
    INSERT INTO media_item_fts(media_item_fts, rowid, title, sort_title, description)
    VALUES ('delete', old.rowid, old.title, old.sort_title, old.description);
END;

CREATE TRIGGER media_item_fts_update
AFTER UPDATE ON media_item BEGIN
    INSERT INTO media_item_fts(media_item_fts, rowid, title, sort_title, description)
    VALUES ('delete', old.rowid, old.title, old.sort_title, old.description);
    INSERT INTO media_item_fts(rowid, title, sort_title, description)
    VALUES (new.rowid, new.title, new.sort_title, new.description);
END;
