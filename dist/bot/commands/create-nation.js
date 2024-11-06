"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
const bot_1 = require("../bot");
const identifiers_1 = require("../identifiers");
const nation_edit_1 = require("../nation-edit");
const database_1 = require("../../server/database");
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('create-nation')
        .setDescription('admin only - create a nation')
        .addStringOption(option => option.setName(identifiers_1.NAME_ID)
        .setDescription("name of the new nation").setRequired(true)).addIntegerOption(option => (0, nation_edit_1.continentChoice)(option, "continent of the new nation").setRequired(true)).addUserOption(option => option.setName(identifiers_1.USER_ID).setDescription("user of the new nation").setRequired(true)),
    async execute(interaction) {
        if (!(0, bot_1.isAdmin)(interaction.member)) {
            interaction.reply({ content: `you are not an admin.`, ephemeral: true });
            return;
        }
        const name = interaction.options.getString(identifiers_1.NAME_ID);
        const continent = database_1.allContinents[interaction.options.getInteger(identifiers_1.CONTINENT_ID)];
        const ownerDiscord = interaction.options.getUser(identifiers_1.USER_ID).id;
        await (0, database_1.createNation)({ name, continent, ownerDiscord, removed: false });
        interaction.reply(`successfully created ${name}`);
    },
    category: "Edit Nation"
};
//# sourceMappingURL=create-nation.js.map