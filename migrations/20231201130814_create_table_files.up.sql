-- Add migration script here
CREATE TABLE IF NOT EXISTS files(
    id UUID PRIMARY KEY,
    link TEXT NOT NULL,
    event_id UUID NOT NULL,
    FOREIGN KEY(event_id) REFERENCES events(id)
);