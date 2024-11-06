"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const nation_edit_1 = require("../nation-edit");
const identifiers_1 = require("../identifiers");
const bot_1 = require("../bot");
const result_1 = require("../../shared/result");
const database = require("../../server/database");
const builders_1 = require("@discordjs/builders");
async function execute(interaction, nation) {
    const mode = modes[interaction.options.getInteger(identifiers_1.MODE_ID)];
    const platform = interaction.options.getString(identifiers_1.PLATFORM_ID).trim();
    let link = interaction.options.getString(identifiers_1.LINK_ID);
    if (mode.link) {
        if (link === null) {
            return (0, result_1.ErrVoid)(`a link is required to ${mode.name} a social`);
        }
        else if (!link.includes("https://")) {
            return (0, result_1.ErrVoid)(`${link} is an invalid link`);
        }
        link = link.trim();
    }
    const result = mode.action(nation.socials, platform, link);
    if (result.err) {
        return (0, result_1.ErrVoid)(result.message);
    }
    else {
        const updated = await database.updateNation({ nationId: nation.nationId, socials: result.value }, (0, bot_1.isAdmin)(interaction.member));
        (0, nation_edit_1.replyOk)(interaction, updated.name);
        return (0, result_1.OkVoid)();
    }
}
function present(socials, platform) {
    return socials.some(social => equal(social, platform));
}
function equal(social, platform) {
    return social.platform.toLowerCase() === platform.toLowerCase();
}
function getIndex(socials, platform) {
    const index = socials.findIndex(social => equal(social, platform));
    if (index < 0) {
        return (0, result_1.Err)(`no social found for "${platform}"`);
    }
    return (0, result_1.Ok)(index);
}
function createSocial(socials, platform, link) {
    if (present(socials, platform)) {
        return (0, result_1.Err)(`the social for "${platform}" is already assigned`);
    }
    const updated = socials;
    updated.push({ link, platform });
    return (0, result_1.Ok)(updated);
}
function updateSocial(socials, platform, link) {
    return getIndex(socials, platform)
        .chain(index => {
        socials[index].link = link;
        return (0, result_1.Ok)(socials);
    });
}
function deleteSocial(socials, platform, _) {
    return getIndex(socials, platform)
        .chain(index => {
        socials.splice(index, 1);
        return (0, result_1.Ok)(socials);
    });
}
const modes = [
    { action: createSocial, name: "create", link: true },
    { action: updateSocial, name: "update", link: true },
    { action: deleteSocial, name: "delete", link: false },
];
const edit = {
    name: "socials",
    execute,
    adminOnly: false
};
module.exports = {
    data: (0, nation_edit_1.editData)(edit, new builders_1.SlashCommandBuilder().addStringOption(option => option.setName(identifiers_1.PLATFORM_ID)
        .setDescription("the platform to edit")
        .setRequired(true)).addIntegerOption(option => {
        option.setName(identifiers_1.MODE_ID).setDescription(`"action to do - specify the "link" option for "create" or "update"`).setRequired(true);
        modes.forEach((mode, index) => {
            option.addChoices({ name: mode.name, value: index });
        });
        return option;
    }).addStringOption(option => option.setName(identifiers_1.LINK_ID)
        .setDescription("the link of the social - required for creation or updating"))),
    execute: (0, nation_edit_1.editExecute)(edit),
    category: "Edit Nation"
};
//# sourceMappingURL=edit-socials.js.map