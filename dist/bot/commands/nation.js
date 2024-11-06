"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
const database = require("../../server/database");
const identifiers_1 = require("../identifiers");
const display_nation_1 = require("../display-nation");
const nation_edit_1 = require("../nation-edit");
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('nation')
        .setDescription('view your nation or a nation specified by name')
        .addStringOption(option => option.setName(identifiers_1.NAME_ID).setRequired(false)
        .setDescription("the name of the ulina nation")),
    async execute(interaction) {
        const name = interaction.options.getString(identifiers_1.NAME_ID);
        const result = name === null
            ? await (0, nation_edit_1.nationByUser)(interaction.member)
            : await database.nationByName(name, database.nationAndSocials);
        if (result.err) {
            interaction.reply({ content: result.message, ephemeral: true });
            return;
        }
        await interaction.reply({ embeds: [await (0, display_nation_1.nationEmbed)(result.value)] });
    },
    category: "View Nation"
};
//# sourceMappingURL=nation.js.map