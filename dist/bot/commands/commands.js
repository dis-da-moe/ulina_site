"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
const bot_1 = require("../bot");
const nation_edit_1 = require("../nation-edit");
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('commands')
        .setDescription('list all commands'),
    async execute(interaction) {
        const categories = new Map();
        for (const command of bot_1.commands) {
            const category = categories.get(command[1].category);
            const displayCommand = { name: command[0], value: command[1].data.description };
            if (category === undefined) {
                categories.set(command[1].category, [displayCommand]);
            }
            else {
                category.push(displayCommand);
            }
        }
        const embed = (0, nation_edit_1.colorEmbed)().setTitle("Moley Commands")
            .setDescription("commands and their descriptions");
        categories.forEach((displayCommands, category) => {
            embed.addField(`__${category}__`, "\u200B");
            for (const command of displayCommands) {
                embed.addField(command.name, command.value, true);
            }
        });
        return interaction.reply({ embeds: [embed] });
    },
    category: "Help"
};
//# sourceMappingURL=commands.js.map