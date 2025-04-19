-- Add migration script here
ALTER TABLE users ALTER COLUMN created_at TYPE TIMESTAMPTZ;