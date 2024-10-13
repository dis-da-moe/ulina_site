-- Add migration script here
CREATE TABLE NewNationChange (
    changeId INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    nationId INTEGER NOT NULL,
    type TEXT NOT NULL,
    oldValue TEXT,
    newValue TEXT,
    timeStamp DATETIME NOT NULL,
    admin BOOLEAN NOT NULL,
    CONSTRAINT NationChange_nationId_fkey FOREIGN KEY (nationId) REFERENCES Nation (nationId) ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO NewNationChange SELECT * FROM NationChange;
DROP TABLE NationChange;
ALTER TABLE NewNationChange RENAME TO NationChange;
UPDATE NationChange SET oldValue = NULL WHERE oldValue = "NULL" OR oldValue = "";
UPDATE NationChange SET newValue = NULL WHERE newValue = "NULL" OR newValue = "";