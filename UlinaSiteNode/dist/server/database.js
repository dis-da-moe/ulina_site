"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.nationByName = exports.allContinents = exports.selectNation = exports.createNation = exports.getMostRecentMap = exports.addMap = exports.selectChanges = exports.updateNation = exports.currentFlag = exports.selectNations = exports.nationExists = exports.nationById = exports.continentExists = exports.validateId = exports.oldNations = exports.currentNations = exports.nationAndSocials = exports.nationIdsNamesContinents = exports.nationIdsNames = exports.prisma = void 0;
const client_1 = require("@prisma/client");
const app_1 = require("./app");
const sync_1 = require("csv-parse/sync");
const utilities_1 = require("./utilities");
const result_1 = require("../shared/result");
const path = require("path");
const fs = require('fs');
exports.prisma = new client_1.PrismaClient();
exports.nationIdsNames = {
    nationId: true,
    name: true
};
exports.nationIdsNamesContinents = {
    nationId: true,
    name: true,
    continent: true
};
exports.nationAndSocials = {
    nationId: true,
    continent: true,
    name: true,
    removed: true,
    description: true,
    currentFlagId: true,
    socials: true,
    ownerDiscord: true
};
exports.currentNations = {
    removed: false,
};
exports.oldNations = {
    removed: true,
};
async function validateId(id) {
    if (id === undefined)
        return (0, result_1.Err)("No ID provided");
    const parsed = parseInt(id.toString(), 10);
    if (isNaN(parsed))
        return (0, result_1.Err)("Id not valid number");
    const nation = await nationExists(nationById(parsed));
    if (nation)
        return (0, result_1.Ok)(parsed);
    else
        return (0, result_1.Err)(`Nation does not exist by id: ${id}`);
}
exports.validateId = validateId;
function continentByName(continentName) {
    return { name: continentName };
}
async function continentExists(continentName) {
    if (continentName === undefined || typeof continentName !== "string")
        return false;
    const continent = await exports.prisma.continent.count({ where: { name: continentName } });
    return continent >= 1;
}
exports.continentExists = continentExists;
function nationById(id) {
    return { nationId: id };
}
exports.nationById = nationById;
async function nationCreationChange(nationId) {
    const input = {
        nation: { connect: nationById(nationId) },
        type: "Creation",
        oldValue: "",
        newValue: "",
        timeStamp: new Date(),
        admin: true,
    };
    await exports.prisma.nationChange.create({ data: input });
}
function parseValue(value) {
    return value === null ? "NULL" : value.toString();
}
async function nationChange(nationId, changeType, oldValue, newValue, admin) {
    const info = {
        nation: { connect: nationById(nationId) },
        type: changeType,
        oldValue: parseValue(oldValue),
        newValue: parseValue(newValue),
        admin: admin,
        timeStamp: new Date()
    };
    await exports.prisma.nationChange.create({ data: info });
}
async function nationExists(where) {
    const count = await exports.prisma.nation.count({ where });
    return count >= 1;
}
exports.nationExists = nationExists;
function selectNations(where, select) {
    return exports.prisma.nation.findMany({
        select: select,
        where: where
    });
}
exports.selectNations = selectNations;
async function currentFlag(where) {
    const nation = await exports.prisma.nation.findUnique({ where });
    if (nation.currentFlagId == null)
        return undefined;
    else {
        const flag = await exports.prisma.flag.findUnique({ where: { flagId: nation.currentFlagId } });
        return flag.flagPath;
    }
}
exports.currentFlag = currentFlag;
async function updateNation(updateInfo, admin = true) {
    const nation = await exports.prisma.nation.findUnique({ where: nationById(updateInfo.nationId), select: exports.nationAndSocials });
    let updateInput = {};
    const change = async (type, oldValue, newValue) => {
        await nationChange(nation.nationId, type, oldValue, newValue, admin);
    };
    if (updateInfo.flag !== undefined) {
        const flag = await createFlag(updateInfo.flag, nation.nationId, nation.name);
        await change("Flag", nation.currentFlagId, flag.flagId);
        updateInput.currentFlagId = flag.flagId;
    }
    if (updateInfo.socials !== undefined) {
        const currentSocials = nation.socials;
        const socialsToAdd = updateInfo.socials.filter(social => social.socialsId === undefined);
        const socialsToUpdate = updateInfo.socials.filter(social => {
            return currentSocials.some(currentSocial => {
                return currentSocial.socialsId === social.socialsId &&
                    (currentSocial.link != social.link || currentSocial.platform != social.platform);
            });
        });
        const socialsToRemove = currentSocials.filter(social => {
            return updateInfo.socials.every(currentSocial => currentSocial.socialsId != social.socialsId);
        });
        for (const social of socialsToAdd) {
            await exports.prisma.social.create({ data: { link: social.link, platform: social.platform, nationId: nation.nationId } });
        }
        for (const social of socialsToRemove) {
            await exports.prisma.social.delete({ where: { socialsId: social.socialsId } });
        }
        for (const social of socialsToUpdate) {
            await exports.prisma.social.update({ where: { socialsId: social.socialsId }, data: { link: social.link, platform: social.platform } });
        }
    }
    if (updateInfo.continent !== undefined) {
        updateInput.continent = { connect: continentByName(updateInfo.continent) };
        await change("Continent", nation.continent.name, updateInfo.continent);
    }
    async function setDifferences(update) {
        if (update[0] !== undefined) {
            await change(update[2], update[1], update[0]);
            update[3](update[0]);
        }
    }
    const normalUpdates = [
        [updateInfo.name, nation.name, "Name", (val) => updateInput.name = val],
        [updateInfo.description, nation.description, "Description", (val) => updateInput.description = val],
        [updateInfo.ownerDiscord, nation.ownerDiscord, "OwnerDiscord", (val) => updateInput.ownerDiscord = val],
        [updateInfo.removed, nation.removed, "Removed", (val) => updateInput.removed = val],
    ];
    for (const update of normalUpdates) {
        await setDifferences(update);
    }
    return await exports.prisma.nation.update({ where: nationById(updateInfo.nationId), data: updateInput });
}
exports.updateNation = updateNation;
async function selectChanges() {
    return await exports.prisma.nationChange.findMany();
}
exports.selectChanges = selectChanges;
async function createFlag(flagInfo, nationId, nationName) {
    const dateString = (0, utilities_1.formatDateForFile)(nationName, flagInfo.fileExtension);
    const location = `/flags/${dateString}`;
    const filePath = path.join(app_1.appDir, `/public/`, location);
    await fs.writeFile(filePath, flagInfo.file, { flag: "w+" }, (err) => { if (err != null)
        console.log(err); });
    const flagInput = {
        flagPath: location,
        nation: { connect: nationById(nationId) }
    };
    return await exports.prisma.flag.create({ data: flagInput });
}
async function addMap(file) {
    const fileName = (0, utilities_1.formatDateForFile)("map", ".svg");
    const filePath = path.join(app_1.appDir, `/data/${fileName}`);
    const result = await (0, utilities_1.writeFile)(filePath, file);
    if (result.ok) {
        await exports.prisma.map.create({ data: {
                fileName: fileName,
                date: new Date()
            } });
    }
    return result;
}
exports.addMap = addMap;
async function getMostRecentMap() {
    const maps = await exports.prisma.map.findMany();
    if (maps.length === 0) {
        return (0, result_1.Err)("No maps found");
    }
    else {
        maps.sort((a, b) => b.date.valueOf() - a.date.valueOf());
        const filePath = `/data/${maps[0].fileName}`;
        return await (0, utilities_1.readFile)(filePath);
    }
}
exports.getMostRecentMap = getMostRecentMap;
async function createNation(nationInfo) {
    const nationInput = {
        continent: { connect: { name: nationInfo.continent } },
        name: nationInfo.name,
        removed: nationInfo.removed,
        description: nationInfo.description,
        ownerDiscord: nationInfo.ownerDiscord,
    };
    const nation = await exports.prisma.nation.create({ data: nationInput });
    await nationCreationChange(nation.nationId);
    const connectNation = { connect: nationById(nation.nationId) };
    if (nationInfo.flag) {
        const flag = await createFlag(nationInfo.flag, nation.nationId, nation.name);
        await exports.prisma.nation.update({ where: nationById(nation.nationId), data: { currentFlagId: flag.flagId } });
    }
    if (nationInfo.socials) {
        for (const socialInfo of nationInfo.socials) {
            const socialInput = {
                nation: connectNation,
                platform: socialInfo.platform,
                link: socialInfo.link
            };
            await exports.prisma.social.create({ data: socialInput });
        }
    }
}
exports.createNation = createNation;
async function selectNation(where, select) {
    return await exports.prisma.nation.findUnique({
        select: select,
        where: where
    });
}
exports.selectNation = selectNation;
function readCsv(path) {
    const fileContent = fs.readFileSync(path);
    return (0, sync_1.parse)(fileContent, { columns: true });
}
exports.allContinents = [
    "Ripiero",
    "Kanita",
    "Zapita",
    "Ailou",
    "Sivalat"
];
async function nationByName(name, select, includeDead = false) {
    name = name.trim();
    const result = await selectNations({
        name: {
            in: [name, (0, utilities_1.capitalise)(name), name.toLowerCase()]
        },
        removed: includeDead
    }, select);
    if (result.length < 1) {
        return (0, result_1.Err)(`nation by the name of "${name}" not found - for the list of nations use \`/all-nations\``);
    }
    const nation = result[0];
    return (0, result_1.Ok)(nation);
}
exports.nationByName = nationByName;
async function createContinents() {
    const continentPath = "data/original/continents.csv";
    const continents = readCsv(path.join(app_1.appDir, continentPath));
    await exports.prisma.continent.deleteMany({ where: { name: { endsWith: "" } } });
    continents.forEach((async (continent) => {
        await exports.prisma.continent.create({ data: { description: continent.description, name: continent.name } });
    }));
}
async function resetDatabase() {
    const nationsCsv = "/data/original/nations.csv";
    const oldNations = readCsv(path.join(app_1.appDir, nationsCsv));
    await exports.prisma.$executeRaw `DELETE FROM Social;`;
    await exports.prisma.$executeRaw `DELETE FROM Flag;`;
    await exports.prisma.$executeRaw `DELETE FROM NationChange;`;
    await exports.prisma.$executeRaw `DELETE FROM Nation;`;
    const nations = oldNations.map((oldRecord) => {
        const socials = [];
        [[oldRecord.Wiki, "Wiki"], [oldRecord.Insta, "Insta"], [oldRecord.LinkToExtra, oldRecord.ExtraSite]]
            .forEach((social) => {
            if (social[0] !== '') {
                socials.push({ platform: social[1], link: social[0], socialsId: undefined });
            }
        });
        return {
            continent: oldRecord.Continent,
            name: oldRecord.Country,
            description: oldRecord.Description == "" ? undefined : oldRecord.Description,
            removed: false,
            ownerDiscord: "",
            socials: socials
        };
    });
    for (const nation of nations) {
        await createNation(nation);
    }
}
//# sourceMappingURL=database.js.map