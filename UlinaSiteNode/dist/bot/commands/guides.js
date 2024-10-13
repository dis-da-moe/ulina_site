"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const fs = require("fs");
const builders_1 = require("@discordjs/builders");
const identifiers_1 = require("../identifiers");
const utilities_1 = require("../../server/utilities");
const nation_edit_1 = require("../nation-edit");
const guides = JSON.parse(fs.readFileSync("./data/guide.json", "utf-8")).guides;
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('guides')
        .setDescription('view guides for getting acquainted with Ulina')
        .addIntegerOption(option => {
        option.setName(identifiers_1.GUIDE_ID).setDescription("the guide to view").setRequired(true);
        guides.forEach((guide, index) => option
            .addChoices({ name: (0, utilities_1.capitalise)(guide.name), value: index }));
        return option;
    }),
    async execute(interaction) {
        const guide = guides[interaction.options.getInteger(identifiers_1.GUIDE_ID)];
        const embed = (0, nation_edit_1.colorEmbed)()
            .setTitle(guide.title)
            .setURL(guide.link)
            .setDescription(`${guide.link}\n${guide.description}`)
            .setImage(guide.image).addField("This includes: ", "\u200B");
        guide.includes.forEach(include => {
            embed.addField(`• **${(0, utilities_1.capitalise)(include)}**`, "\u200B", true);
        });
        interaction.reply({ embeds: [embed] });
    },
    category: "Help"
};
//# sourceMappingURL=guides.js.map