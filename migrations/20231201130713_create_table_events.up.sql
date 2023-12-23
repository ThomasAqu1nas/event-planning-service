-- Add migration script here
CREATE TABLE IF NOT EXISTS events(
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    descr TEXT NOT NULL,
    dt TIMESTAMPTZ NOT NULL,
    place TEXT,
    creator UUID NOT NULL,
    FOREIGN KEY(creator) REFERENCES users(id)
);