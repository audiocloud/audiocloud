-- Add migration script here
ALTER TABLE media_object
    ADD COLUMN revision INTEGER NOT NULL DEFAULT 0;