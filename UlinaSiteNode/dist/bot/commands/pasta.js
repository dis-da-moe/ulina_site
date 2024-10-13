"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
const fs = require("fs");
const pastas = JSON.parse(fs.readFileSync("./data/pasta.json", "utf-8")).pastas;
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('pasta')
        .setDescription('sends the ulina copy-pasta of your choice')
        .addIntegerOption(input => {
        const option = input.setName("pasta").setRequired(true)
            .setDescription("the selected pasta");
        pastas.forEach((pasta, index) => {
            option.addChoices({ name: pasta.name, value: index });
        });
        return option;
    }),
    async execute(interaction) {
        const pasta = pastas[interaction.options.getInteger("pasta")];
        await interaction.reply(pasta.content);
    },
    category: "Misc"
};
//# sourceMappingURL=pasta.js.map