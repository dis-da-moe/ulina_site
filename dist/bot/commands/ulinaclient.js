"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('ulinaclient')
        .setDescription('Ulina Client'),
    async execute(interaction) {
        await interaction.reply({
            content: "Hatsune Miku says: \"Wow! Ulinaclient is so cool! Download it today using Ulina Ulownloader!\"",
            files: [{
                    attachment: "https://cdn.discordapp.com/attachments/960224027765514360/976906175406809138/ulinaclient.mp3",
                    name: "ulinaclient.mp3"
                }]
        });
    },
    category: "Misc"
};
//# sourceMappingURL=ulinaclient.js.map