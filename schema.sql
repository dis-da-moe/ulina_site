CREATE TABLE sqlite_sequence(name,seq);
CREATE TABLE IF NOT EXISTS "Continent" (
    "name" TEXT NOT NULL PRIMARY KEY,
    "description" TEXT
);
CREATE TABLE IF NOT EXISTS "Flag" (
    "flagId" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "flagPath" TEXT NOT NULL,
    "nationId" INTEGER NOT NULL,
    CONSTRAINT "Flag_nationId_fkey" FOREIGN KEY ("nationId") REFERENCES "Nation" ("nationId") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE IF NOT EXISTS "Social" (
    "socialsId" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "nationId" INTEGER NOT NULL,
    "link" TEXT NOT NULL,
    "platform" TEXT NOT NULL,
    CONSTRAINT "Social_nationId_fkey" FOREIGN KEY ("nationId") REFERENCES "Nation" ("nationId") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE IF NOT EXISTS "Nation" (
    "nationId" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "continentName" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "removed" BOOLEAN NOT NULL DEFAULT false,
    "ownerDiscord" TEXT NOT NULL,
    "description" TEXT,
    "currentFlagId" INTEGER, leader TEXT, capital TEXT, ideology TEXT, alliances TEXT,
    CONSTRAINT "Nation_continentName_fkey" FOREIGN KEY ("continentName") REFERENCES "Continent" ("name") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE IF NOT EXISTS "Map" (
    "mapId" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "fileName" TEXT NOT NULL,
    "date" DATETIME NOT NULL
);
CREATE TABLE IF NOT EXISTS "Session" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "sid" TEXT NOT NULL,
    "data" TEXT NOT NULL,
    "expiresAt" DATETIME NOT NULL
);
CREATE UNIQUE INDEX "Session_sid_key" ON "Session"("sid");
CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
CREATE TABLE IF NOT EXISTS "NationChange" (
    changeId INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    nationId INTEGER NOT NULL,
    type TEXT NOT NULL,
    oldValue TEXT,
    newValue TEXT,
    timeStamp DATETIME NOT NULL,
    admin BOOLEAN NOT NULL,
    CONSTRAINT NationChange_nationId_fkey FOREIGN KEY (nationId) REFERENCES Nation (nationId) ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE User(
    userId INTEGER PRIMARY KEY,
    isAdmin BOOLEAN NOT NULL,
    discord TEXT,
    pendingAuth TEXT
, lastVisit DATETIME NOT NULL);