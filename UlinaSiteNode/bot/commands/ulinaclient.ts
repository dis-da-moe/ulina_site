import {SlashCommandBuilder} from '@discordjs/builders';
import {CommandInteraction} from "discord.js";
import {UlinaCommand} from "../bot";

module.exports = {
    data: new SlashCommandBuilder()
        .setName('ulinaclient')
        .setDescription('Ulina Client'),
    async execute(interaction: CommandInteraction) {
        await interaction.reply({
            content: "Hatsune Miku says: \"Wow! Ulinaclient is so cool! Download it today using Ulina Ulownloader!\"",
            files: [{
                attachment: "https://cdn.discordapp.com/attachments/960224027765514360/976906175406809138/ulinaclient.mp3",
                name: "ulinaclient.mp3"
            }]
        });
    },
    category: "Misc"
} as UlinaCommand;