import {SlashCommandBuilder} from '@discordjs/builders';
import * as database from "../../server/database";
import {CommandInteraction} from "discord.js";
import {UlinaCommand} from "../bot";
import {NAME_ID} from "../identifiers";
import {NationAndSocials} from "../../server/database";
import {nationEmbed} from "../display-nation";
import {nationByUser} from "../nation-edit";

module.exports = {
    data: new SlashCommandBuilder()
        .setName('nation')
        .setDescription('view your nation or a nation specified by name')
        .addStringOption(option =>
            option.setName(NAME_ID).setRequired(false)
                .setDescription("the name of the ulina nation")
        ),
    async execute(interaction: CommandInteraction) {
        const name = interaction.options.getString(NAME_ID);

        const result = name === null
            ? await nationByUser(interaction.member)
            : await database.nationByName<NationAndSocials>(name, database.nationAndSocials);

        if (result.err){
            interaction.reply({content: result.message, ephemeral: true});
            return;
        }

        await interaction.reply({embeds: [await nationEmbed(result.value)]});
    },
    category: "View Nation"
} as UlinaCommand;