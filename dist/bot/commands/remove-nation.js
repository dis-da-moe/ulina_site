"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const database_1 = require("../../server/database");
const nation_edit_1 = require("../nation-edit");
const result_1 = require("../../shared/result");
const builders_1 = require("@discordjs/builders");
async function execute(interaction, nation) {
    await (0, database_1.updateNation)({ nationId: nation.nationId, removed: true });
    interaction.reply(`successfully removed ${nation.name}`);
    return (0, result_1.OkVoid)();
}
const edit = {
    adminOnly: true,
    execute,
    name: "remove nation"
};
module.exports = {
    data: (0, nation_edit_1.editData)(edit, new builders_1.SlashCommandBuilder())
        .setDescription('admin only - remove a nation'),
    execute: (0, nation_edit_1.editExecute)(edit),
    category: "Edit Nation"
};
//# sourceMappingURL=remove-nation.js.map