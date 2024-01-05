-- Add up migration script here
CREATE TABLE IF NOT EXISTS notifications(
   id UUID PRIMARY KEY,
   recipient UUID NOT NULL,
   content TEXT,
   stat INTEGER NOT NULL,
   creation_dt TIMESTAMPTZ NOT NULL,
   sending_dt TIMESTAMPTZ,
   FOREIGN KEY(recipient) REFERENCES users(id)
);