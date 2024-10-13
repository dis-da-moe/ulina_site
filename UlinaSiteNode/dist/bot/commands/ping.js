"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('ping')
        .setDescription('Replies with Pong!'),
    async execute(interaction) {
        const message = Math.random() > 0.8 ? "Ping! wait ah fuck sorry i mean Ping! no i'm so sorry i mean Pong! wait no wait yeah i'm sorry fuck Pong!" : "Pong!";
        await interaction.reply(message);
    },
    category: "Misc"
};
//# sourceMappingURL=ping.js.map