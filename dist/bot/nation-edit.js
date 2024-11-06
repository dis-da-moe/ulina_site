"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.continentChoice = exports.editExecute = exports.editData = exports.colorEmbed = exports.nationByUser = exports.validateUser = exports.replyOk = void 0;
const discord_js_1 = require("discord.js");
const database = require("../server/database");
const database_1 = require("../server/database");
const bot_1 = require("./bot");
const identifiers_1 = require("./identifiers");
const result_1 = require("../shared/result");
function replyOk(interaction, name) {
    interaction.reply(`${name} successfully edited`);
}
exports.replyOk = replyOk;
function validateUser(nation, member) {
    member = member;
    const isOwner = nation.ownerDiscord !== "" && nation.ownerDiscord === member.user.id;
    if (!isOwner && !(0, bot_1.isAdmin)(member)) {
        return (0, result_1.Err)(`this nation does not belong to you - if this is a mistake contact the moderators`);
    }
    else {
        return (0, result_1.Ok)(nation);
    }
}
exports.validateUser = validateUser;
async function nationByUser(member) {
    const results = await database.selectNations({ ownerDiscord: member.user.id, removed: false }, database.nationAndSocials);
    if (results.length < 1) {
        return (0, result_1.Err)(`no nation was found linked to your account - ${(0, bot_1.isAdmin)(member) ? "try specifying a name" : "contact moderators if this is a mistake"}`);
    }
    return (0, result_1.Ok)(results[0]);
}
exports.nationByUser = nationByUser;
function colorEmbed() {
    return new discord_js_1.MessageEmbed().setColor("#0f4272");
}
exports.colorEmbed = colorEmbed;
function editData(edit, command) {
    return command
        .setName("edit-" + edit.name.replace(" and ", " ").replaceAll(' ', '-'))
        .setDescription(edit.adminOnly ? `admin only - change the ${edit.name} of a nation` : `change the ${edit.name} of your ulina nation`)
        .addStringOption(input => input.setName(identifiers_1.NAME_ID)
        .setDescription(`${edit.adminOnly ? "" : "admin only -"} name of the ulina country to edit`)
        .setRequired(edit.adminOnly));
}
exports.editData = editData;
function editExecute(edit) {
    return async (interaction) => {
        const name = interaction.options.getString(identifiers_1.NAME_ID);
        if (edit.adminOnly && !(0, bot_1.isAdmin)(interaction.member)) {
            interaction.reply({ content: `only a moderator can change the ${edit.name} of a nation - contact one if this is desired`, ephemeral: true });
            return;
        }
        const nation = name === null
            ? await nationByUser(interaction.member)
            : (await database.nationByName(name, database.nationAndSocials))
                .chain(nation => validateUser(nation, interaction.member));
        const result = await nation.asyncChain(nation => edit.execute(interaction, nation));
        if (result.err) {
            if (interaction.replied) {
                interaction.editReply(result.message);
            }
            else {
                interaction.reply({ content: result.message, ephemeral: true });
            }
        }
    };
}
exports.editExecute = editExecute;
function continentChoice(option, description) {
    option = option.setName(identifiers_1.CONTINENT_ID).setDescription(description);
    database_1.allContinents.forEach((continent, index) => option.addChoices({ name: continent, value: index }));
    return option;
}
exports.continentChoice = continentChoice;
//# sourceMappingURL=nation-edit.js.map