-- Add migration script here
CREATE TABLE User(
    userId INTEGER PRIMARY KEY,
    isAdmin BOOLEAN NOT NULL,
    discord TEXT,
    pendingAuth TEXT
);