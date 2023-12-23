-- Add migration script here
CREATE TABLE IF NOT EXISTS participations(
    event_id UUID NOT NULL,
    user_id UUID NOT NULL,
    PRIMARY KEY(event_id, user_id)
);