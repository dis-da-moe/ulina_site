"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
const timeUtil_1 = require("../../shared/timeUtil");
const nation_edit_1 = require("../nation-edit");
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('current-time')
        .setDescription('gives current ulina time'),
    async execute(interaction) {
        const embed = (0, nation_edit_1.colorEmbed)()
            .addField("Real Time: ", (new Date()).toDateString())
            .addField("Ulina Time: ", (0, timeUtil_1.ulinaTimeNow)().toDateString());
        await interaction.reply({ embeds: [embed] });
    },
    category: "Time"
};
//# sourceMappingURL=current-time.js.map