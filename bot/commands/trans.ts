import {SlashCommandBuilder} from '@discordjs/builders';
import {CommandInteraction} from "discord.js";
import {UlinaCommand} from "../bot";
const trans = "🏳️‍⚧️";
module.exports = {
    data: new SlashCommandBuilder()
        .setName('trans')
        .setDescription('Trans Rights'),
    async execute(interaction: CommandInteraction) {
        await interaction.reply(`${trans} TRANS ${trans} RIGHTS ${trans} ARE ${trans} HUMAN ${trans} RIGHTS ${trans}`);
    },
    category: "Misc"
} as UlinaCommand;