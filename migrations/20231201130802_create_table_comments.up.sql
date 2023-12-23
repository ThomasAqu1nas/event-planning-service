-- Add migration script here
CREATE TABLE IF NOT EXISTS comments(
    id UUID PRIMARY KEY,
    comment TEXT NOT NULL,
    event_id UUID NOT NULL,
    user_id UUID NOT NULL,
    FOREIGN KEY(event_id) REFERENCES events(id),
    FOREIGN KEY(user_id) REFERENCES users(id)
);