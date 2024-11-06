import {SlashCommandBuilder} from '@discordjs/builders';
import {CommandInteraction} from "discord.js";
import {UlinaCommand} from "../bot";

module.exports = {
    data: new SlashCommandBuilder()
        .setName('pong')
        .setDescription('Replies with Ping!'),
    async execute(interaction: CommandInteraction) {
        const message = Math.random() > 0.8 ? "<:trout:883701391489634314>" : "Ping!";
        await interaction.reply(message);
    },
    category: "Misc"
} as UlinaCommand;