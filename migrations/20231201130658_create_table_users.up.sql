-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
    id UUID PRIMARY KEY,
    username VARCHAR(24) NOT NULL,
    pwd_hash VARCHAR(64) NOT NULL,
    email VARCHAR(50),
    access_token VARCHAR(50),
    refresh_token VARCHAR(50) 
);