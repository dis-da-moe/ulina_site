import {SlashCommandBuilder} from '@discordjs/builders';
import * as database from "../../server/database";
import {CommandInteraction, Message, MessageActionRow, MessageButton, MessageEmbed} from "discord.js";
import {UlinaCommand} from "../bot";
import {CONTINENT_ID, NEXT_ID, PREVIOUS_ID} from "../identifiers";
import {allContinents, NationAndSocials} from "../../server/database";
import {colorEmbed, continentChoice} from "../nation-edit";
import {displayUser} from "../display-nation";

module.exports = {
    data: new SlashCommandBuilder()
        .setName('all-nations')
        .setDescription('view a list of nations')
        .addIntegerOption(option =>
            continentChoice(option, "limit nations to a continent")
        ),
    async execute(interaction: CommandInteraction) {
        const nations = await database.selectNations(database.currentNations, database.nationAndSocials) as  NationAndSocials[];
        const continentInput = interaction.options.getInteger(CONTINENT_ID);
        const searchContinent = continentInput === null ? null : allContinents[continentInput];

        const continents = new Map<string, MessageEmbed>();

        nations.forEach(nation => {
            let continent = continents.get(nation.continent.name);
            if (continent === undefined){
                continent = colorEmbed()
                    .setTitle(`Nations`)
                    .setDescription(`${nation.continent.name}`);
                continents.set(nation.continent.name, continent);
            }

            continent.addField(nation.name,displayUser(nation),true);
        });

        let embeds: MessageEmbed[] = [];
        if (searchContinent !== null){
            const result = continents.get(searchContinent);
            if (result === undefined){
                interaction.reply(`no nations found for ${searchContinent}`);
                return;
            }

            embeds.push(result);
            await interaction.reply({embeds});
            return;
        }
        else{
            embeds = Array.from(continents.values());
        }

        const getContent = (page: number) => {
            const row = new MessageActionRow();

            row.addComponents(
                new MessageButton()
                    .setCustomId(PREVIOUS_ID)
                    .setStyle('SECONDARY')
                    .setEmoji('⬅️')
                    .setDisabled(page === 0)
            );

            row.addComponents(
                new MessageButton()
                    .setCustomId(NEXT_ID)
                    .setStyle('SECONDARY')
                    .setEmoji('➡️')
                    .setDisabled(page === embeds.length - 1)
            );

            return {embeds: [embeds[page]], components: [row]};
        };

        let page = 0;

        await interaction.reply(getContent(page));

        const reply = await interaction.fetchReply() as unknown as Message;

        const filter = i => {
            i.deferUpdate();
            return i.user.id === interaction.user.id;
        }
        const timeout = 60 * 1000;
        const collector = reply.createMessageComponentCollector({ filter, componentType: 'BUTTON', time: timeout });
        collector.on('collect', i => {
            if(i.customId === PREVIOUS_ID){
                page -= 1;
            }
            else if(i.customId === NEXT_ID){
                page += 1;
            }
            else{
                i.deferUpdate();
                return;
            }
            interaction.editReply(getContent(page));
        });
        setTimeout(() => {
            interaction.editReply({embeds: [embeds[page]], components: []});
        }, timeout);
    },
    category: "View Nation"
} as UlinaCommand;