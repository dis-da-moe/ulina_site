"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const discord_js_1 = require("discord.js");
const nation_edit_1 = require("../nation-edit");
const identifiers_1 = require("../identifiers");
const result_1 = require("../../shared/result");
const builders_1 = require("@discordjs/builders");
async function execute(interaction, nation) {
    const modal = new discord_js_1.Modal().setCustomId(nation.nationId.toString()).setTitle(`Edit ${nation.name}`);
    const nameInput = new discord_js_1.TextInputComponent()
        .setCustomId(identifiers_1.NAME_INPUT_ID)
        .setLabel("Name")
        .setRequired(true)
        .setValue(nation.name)
        .setStyle("SHORT");
    const descriptionInput = new discord_js_1.TextInputComponent()
        .setCustomId(identifiers_1.DESCRIPTION_INPUT_ID)
        .setLabel("Description")
        .setRequired(false)
        .setStyle("PARAGRAPH");
    if (nation.description !== null) {
        descriptionInput.setValue(nation.description);
    }
    [nameInput, descriptionInput].forEach(input => {
        modal.addComponents(new discord_js_1.MessageActionRow().addComponents(input));
    });
    await interaction.showModal(modal);
    return (0, result_1.OkVoid)();
}
const edit = {
    name: "name and description",
    execute,
    adminOnly: false
};
module.exports = {
    data: (0, nation_edit_1.editData)(edit, new builders_1.SlashCommandBuilder()),
    execute: (0, nation_edit_1.editExecute)(edit),
    category: "Edit Nation"
};
//# sourceMappingURL=edit-name-description.js.map