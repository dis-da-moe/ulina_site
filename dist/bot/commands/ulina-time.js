"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
const timeUtil_1 = require("../../shared/timeUtil");
const nation_edit_1 = require("../nation-edit");
const commmand = new builders_1.SlashCommandBuilder()
    .setName('ulina-time')
    .addIntegerOption(option => option.setName("year")
    .setDescription("Year of the date to be converted from")
    .setRequired(true)
    .setMinValue(2019))
    .addIntegerOption(option => option.setName("month").setRequired(true)
    .setDescription("Month of the date to be converted from")
    .setMinValue(1)
    .setMaxValue(12))
    .addIntegerOption(option => option.setName("day").setRequired(false)
    .setDescription("Day of the date to be converted from (optional)")
    .setMinValue(1)
    .setMaxValue(31))
    .setDescription('converts the given date in real time to ulina time');
module.exports = {
    data: commmand,
    async execute(interaction) {
        const year = interaction.options.getInteger("year");
        const month = interaction.options.getInteger("month");
        let day = interaction.options.getInteger("day");
        if (day === null)
            day = 1;
        if (!(0, timeUtil_1.isValidDate)(year, month, day)) {
            await interaction.reply({ content: `${year}/${month}/${day} is not a valid date`, ephemeral: true });
            return;
        }
        const date = new Date(Date.UTC(year, month - 1, day));
        if (!(0, timeUtil_1.realTimeInRange)(date)) {
            await interaction.reply({ content: `${date.toDateString()} is before Ulina Time was tracked reliably, so a conversion is not possible.`, ephemeral: true });
            return;
        }
        const converted = (0, timeUtil_1.toUlinaTime)(date.valueOf());
        const embed = (0, nation_edit_1.colorEmbed)()
            .addField("Real Time: ", date.toDateString())
            .addField("Ulina Time: ", converted.toDateString());
        await interaction.reply({ embeds: [embed] });
    },
    category: "Time"
};
//# sourceMappingURL=ulina-time.js.map