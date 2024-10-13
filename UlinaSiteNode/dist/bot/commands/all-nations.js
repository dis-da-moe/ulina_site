"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const builders_1 = require("@discordjs/builders");
const database = require("../../server/database");
const discord_js_1 = require("discord.js");
const identifiers_1 = require("../identifiers");
const database_1 = require("../../server/database");
const nation_edit_1 = require("../nation-edit");
const display_nation_1 = require("../display-nation");
module.exports = {
    data: new builders_1.SlashCommandBuilder()
        .setName('all-nations')
        .setDescription('view a list of nations')
        .addIntegerOption(option => (0, nation_edit_1.continentChoice)(option, "limit nations to a continent")),
    async execute(interaction) {
        const nations = await database.selectNations(database.currentNations, database.nationAndSocials);
        const continentInput = interaction.options.getInteger(identifiers_1.CONTINENT_ID);
        const searchContinent = continentInput === null ? null : database_1.allContinents[continentInput];
        const continents = new Map();
        nations.forEach(nation => {
            let continent = continents.get(nation.continent.name);
            if (continent === undefined) {
                continent = (0, nation_edit_1.colorEmbed)()
                    .setTitle(`Nations`)
                    .setDescription(`${nation.continent.name}`);
                continents.set(nation.continent.name, continent);
            }
            continent.addField(nation.name, (0, display_nation_1.displayUser)(nation), true);
        });
        let embeds = [];
        if (searchContinent !== null) {
            const result = continents.get(searchContinent);
            if (result === undefined) {
                interaction.reply(`no nations found for ${searchContinent}`);
                return;
            }
            embeds.push(result);
            await interaction.reply({ embeds });
            return;
        }
        else {
            embeds = Array.from(continents.values());
        }
        const getContent = (page) => {
            const row = new discord_js_1.MessageActionRow();
            row.addComponents(new discord_js_1.MessageButton()
                .setCustomId(identifiers_1.PREVIOUS_ID)
                .setStyle('SECONDARY')
                .setEmoji('⬅️')
                .setDisabled(page === 0));
            row.addComponents(new discord_js_1.MessageButton()
                .setCustomId(identifiers_1.NEXT_ID)
                .setStyle('SECONDARY')
                .setEmoji('➡️')
                .setDisabled(page === embeds.length - 1));
            return { embeds: [embeds[page]], components: [row] };
        };
        let page = 0;
        await interaction.reply(getContent(page));
        const reply = await interaction.fetchReply();
        const filter = i => {
            i.deferUpdate();
            return i.user.id === interaction.user.id;
        };
        const timeout = 60 * 1000;
        const collector = reply.createMessageComponentCollector({ filter, componentType: 'BUTTON', time: timeout });
        collector.on('collect', i => {
            if (i.customId === identifiers_1.PREVIOUS_ID) {
                page -= 1;
            }
            else if (i.customId === identifiers_1.NEXT_ID) {
                page += 1;
            }
            else {
                i.deferUpdate();
                return;
            }
            interaction.editReply(getContent(page));
        });
        setTimeout(() => {
            interaction.editReply({ embeds: [embeds[page]], components: [] });
        }, timeout);
    },
    category: "View Nation"
};
//# sourceMappingURL=all-nations.js.map