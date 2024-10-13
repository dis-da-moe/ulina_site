"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const fs = require("fs");
const utilities_1 = require("./utilities");
const backUpFilePath = "./data/backups/backup.json";
const interval = 86400000 * 7;
const start = () => {
    const now = Date.now();
    const lastBackUp = JSON.parse(fs.readFileSync(backUpFilePath, "utf-8")).lastBackUp;
    const tillNextBackUp = lastBackUp === null ? 0 : Math.max(0, lastBackUp - now + interval);
    console.log(`next backup scheduled for ${new Date(now + tillNextBackUp)}`);
    setTimeout(backUp, tillNextBackUp);
};
async function backUp() {
    const now = Date.now();
    console.log(`${new Date(now)} - backing up...`);
    fs.writeFileSync(backUpFilePath, JSON.stringify({ lastBackUp: now }));
    fs.copyFile("./data/Ulina.db", `./data/backups/${(0, utilities_1.formatDateForFile)("Ulina", ".db")}`, (err) => {
        if (err !== null)
            console.log(`error backing up database: ${err}`);
        else {
            console.log("successful backup");
        }
        setTimeout(backUp, interval);
        console.log(`next backup scheduled for ${new Date(now + interval)}`);
    });
}
start();
//# sourceMappingURL=backup.js.map