import {SlashCommandBuilder} from '@discordjs/builders';
import {isValidDate, toUlinaTime, realTimeInRange} from "../../shared/timeUtil";
import {CommandInteraction, MessageEmbed} from "discord.js";
import {UlinaCommand} from "../bot";
import {colorEmbed} from "../nation-edit";

const commmand = new SlashCommandBuilder()
    .setName('ulina-time')
    .addIntegerOption(option =>
        option.setName("year")
            .setDescription("Year of the date to be converted from")
            .setRequired(true)
            .setMinValue(2019)
    )
    .addIntegerOption(option =>
        option.setName("month").setRequired(true)
            .setDescription("Month of the date to be converted from")
            .setMinValue(1)
            .setMaxValue(12)
    )
    .addIntegerOption(option =>
        option.setName("day").setRequired(false)
            .setDescription("Day of the date to be converted from (optional)")
            .setMinValue(1)
            .setMaxValue(31)
    )
    .setDescription('converts the given date in real time to ulina time');

module.exports = {
    data: commmand,
    async execute(interaction: CommandInteraction) {
        const year = interaction.options.getInteger("year");
        const month = interaction.options.getInteger("month");
        let day = interaction.options.getInteger("day");
        if (day === null) day = 1;

        if (!isValidDate(year, month, day)){
            await interaction.reply({content: `${year}/${month}/${day} is not a valid date`, ephemeral: true});
            return;
        }

        const date = new Date(Date.UTC(year, month - 1, day));

        if (!realTimeInRange(date)){
            await interaction.reply({content: `${date.toDateString()} is before Ulina Time was tracked reliably, so a conversion is not possible.`, ephemeral: true});
            return;
        }
        const converted = toUlinaTime(date.valueOf());

        const embed = colorEmbed()
            .addField("Real Time: ", date.toDateString())
            .addField("Ulina Time: ", converted.toDateString());

        await interaction.reply({embeds: [embed]});
    },
    category: "Time"
} as UlinaCommand;