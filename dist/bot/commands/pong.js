"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('pong')
        .setDescription('Replies with Ping!'),
    async execute(interaction) {
        const message = Math.random() > 0.8 ? "<:trout:883701391489634314>" : "Ping!";
        await interaction.reply(message);
    },
    category: "Misc"
};
//# sourceMappingURL=pong.js.map