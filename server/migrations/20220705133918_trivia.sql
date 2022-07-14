-- Add migration script here
ALTER TABLE Nation ADD COLUMN leader TEXT;
ALTER TABLE Nation ADD COLUMN capital TEXT;
ALTER TABLE Nation ADD COLUMN ideology TEXT;
ALTER TABLE Nation ADD COLUMN alliances TEXT;
