"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
const trans = "🏳️‍⚧️";
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('trans')
        .setDescription('Trans Rights'),
    async execute(interaction) {
        await interaction.reply(`${trans} TRANS ${trans} RIGHTS ${trans} ARE ${trans} HUMAN ${trans} RIGHTS ${trans}`);
    },
    category: "Misc"
};
//# sourceMappingURL=trans.js.map