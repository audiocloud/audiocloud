-- Add migration script here

CREATE TABLE sys_props
(
    id    TEXT NOT NULL PRIMARY KEY,
    value TEXT NOT NULL
) STRICT;

CREATE TABLE media_objects
(
    id         TEXT NOT NULL PRIMARY KEY,
    path       TEXT NULL     DEFAULT NULL,
    metadata   TEXT NULL     DEFAULT NULL,
    last_used  TEXT NOT NULL,
    created_at TEXT NOT NULL,
    revision   INT  NOT NULL DEFAULT 0
) STRICT;

CREATE INDEX media_object_last_used_idx ON media_objects (last_used);

CREATE TABLE media_jobs
(
    id          TEXT NOT NULL PRIMARY KEY,
    upload      TEXT NULL     DEFAULT NULL,
    download    TEXT NULL     DEFAULT NULL,
    state       TEXT NOT NULL,
    in_progress INT  NOT NULL DEFAULT FALSE,
    updated_at  TEXT NOT NULL,
    created_at  TEXT NOT NULL,
    media_id    TEXT REFERENCES media_objects (id) ON DELETE SET NULL
) STRICT;

CREATE INDEX media_job_updated_at_idx ON media_jobs (updated_at);
CREATE INDEX media_job_downloads_in_progress_idx ON media_jobs (in_progress) WHERE download IS NOT NULL;
CREATE INDEX media_job_uploads_in_progress_idx ON media_jobs (in_progress) WHERE upload IS NOT NULL;
CREATE INDEX media_job_media_id ON media_jobs (media_id);

CREATE TABLE models
(
    id   TEXT NOT NULL PRIMARY KEY,
    spec TEXT NOT NULL
) STRICT;
