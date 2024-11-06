import {SlashCommandBuilder} from '@discordjs/builders';
import {CommandInteraction} from "discord.js";
import {UlinaCommand} from "../bot";

module.exports = {
    data: new SlashCommandBuilder()
        .setName('ping')
        .setDescription('Replies with Pong!'),
    async execute(interaction: CommandInteraction) {
        const message = Math.random() > 0.8 ? "Ping! wait ah fuck sorry i mean Ping! no i'm so sorry i mean Pong! wait no wait yeah i'm sorry fuck Pong!" : "Pong!";
        await interaction.reply(message);
    },
    category: "Misc"
} as UlinaCommand;