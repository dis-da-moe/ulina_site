"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const nation_edit_1 = require("../nation-edit");
const identifiers_1 = require("../identifiers");
const bot_1 = require("../bot");
const database = require("../../server/database");
const result_1 = require("../../shared/result");
const utilities_1 = require("../../server/utilities");
const axios_1 = require("axios");
const builders_1 = require("@discordjs/builders");
async function execute(interaction, nation) {
    const attachment = interaction.options.getAttachment(identifiers_1.FLAG_ID);
    const extension = (0, utilities_1.matchesImageExtension)(attachment.name);
    if (extension.err) {
        return (0, result_1.ErrVoid)(`${attachment.name} is not an image file`);
    }
    await interaction.reply({ content: `updating...`, ephemeral: true });
    const response = await axios_1.default.get(attachment.url, { responseType: 'arraybuffer' });
    const info = {
        nationId: nation.nationId,
        flag: {
            file: Buffer.from(response.data),
            fileExtension: extension.value
        }
    };
    const updated = await database.updateNation(info, (0, bot_1.isAdmin)(interaction.member));
    interaction.followUp(`edited flag of ${updated.name} successfully`);
    return (0, result_1.OkVoid)();
}
const edit = {
    name: "flag",
    execute,
    adminOnly: false
};
module.exports = {
    data: (0, nation_edit_1.editData)(edit, new builders_1.SlashCommandBuilder().addAttachmentOption(option => option.setName(identifiers_1.FLAG_ID)
        .setDescription("the new flag of the country")
        .setRequired(true))),
    execute: (0, nation_edit_1.editExecute)(edit),
    category: "Edit Nation"
};
//# sourceMappingURL=edit-flag.js.map