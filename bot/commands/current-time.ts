import {SlashCommandBuilder} from '@discordjs/builders';
import {ulinaTimeNow} from "../../shared/timeUtil";
import {CommandInteraction, MessageEmbed} from "discord.js";
import {UlinaCommand} from "../bot";
import {colorEmbed} from "../nation-edit";

module.exports = {
    data: new SlashCommandBuilder()
        .setName('current-time')
        .setDescription('gives current ulina time'),
    async execute(interaction: CommandInteraction) {
        const embed = colorEmbed()
            .addField("Real Time: ", (new Date()).toDateString())
            .addField("Ulina Time: ", ulinaTimeNow().toDateString());
        await interaction.reply({embeds: [embed]});
    },
    category: "Time"
} as UlinaCommand;