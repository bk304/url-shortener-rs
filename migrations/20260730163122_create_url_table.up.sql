-- Add up migration script here
CREATE TABLE urls (
    token VARCHAR(6) PRIMARY KEY,
    original_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NULL DEFAULT NOW()
);