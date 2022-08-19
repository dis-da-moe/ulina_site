-- Add migration script here
DELETE FROM User;
ALTER TABLE User ADD COLUMN lastVisit DATETIME NOT NULL;