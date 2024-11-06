"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const database = require("../../server/database");
const nation_edit_1 = require("../nation-edit");
const identifiers_1 = require("../identifiers");
const result_1 = require("../../shared/result");
const builders_1 = require("@discordjs/builders");
async function execute(interaction, nation) {
    const user = interaction.options.getUser(identifiers_1.USER_ID);
    const info = {
        nationId: nation.nationId,
        ownerDiscord: user.id
    };
    const updated = await database.updateNation(info, true);
    (0, nation_edit_1.replyOk)(interaction, updated.name);
    return (0, result_1.OkVoid)();
}
const edit = {
    name: "discord user",
    execute,
    adminOnly: true
};
module.exports = {
    data: (0, nation_edit_1.editData)(edit, new builders_1.SlashCommandBuilder())
        .addUserOption(input => input.setName(identifiers_1.USER_ID)
        .setDescription("the user that owns this nation")
        .setRequired(true)),
    execute: (0, nation_edit_1.editExecute)(edit),
    category: "Edit Nation"
};
//# sourceMappingURL=edit-discord-user.js.map