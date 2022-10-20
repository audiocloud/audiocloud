-- Add migration script here

CREATE TABLE sys_props
(
    id    TEXT NOT NULL PRIMARY KEY,
    value TEXT NOT NULL
) STRICT;

CREATE TABLE media_object
(
    id        TEXT NOT NULL PRIMARY KEY,
    path      TEXT,
    metadata  TEXT,
    last_used TEXT NOT NULL
) STRICT;

CREATE INDEX media_object_last_used_idx ON media_object (last_used);

CREATE TABLE media_job
(
    id            TEXT NOT NULL PRIMARY KEY,
    kind          TEXT NOT NULL,
    spec          TEXT NOT NULL,
    state         TEXT NOT NULL,
    last_modified TEXT NOT NULL,
    active        INT  NOT NULL DEFAULT FALSE,
    media_id      TEXT REFERENCES media_object (id) ON DELETE SET NULL
) STRICT;

CREATE INDEX media_job_last_modified_idx ON media_job (last_modified);
CREATE INDEX media_job_kind_active ON media_job (kind, active);
CREATE INDEX media_job_media_id ON media_job (media_id);

CREATE TABLE model
(
    id   TEXT NOT NULL PRIMARY KEY,
    spec TEXT NOT NULL
) STRICT;
