import {SlashCommandBuilder} from '@discordjs/builders';
import {CommandInteraction} from "discord.js";
import {isAdmin, UlinaCommand} from "../bot";
import {CONTINENT_ID, FLAG_ID, NAME_ID, USER_ID} from "../identifiers";
import {continentChoice} from "../nation-edit";
import {allContinents, createNation} from "../../server/database";

module.exports = {
    data: new SlashCommandBuilder()
        .setName('create-nation')
        .setDescription('admin only - create a nation')
        .addStringOption(option =>
            option.setName(NAME_ID)
                .setDescription("name of the new nation").setRequired(true)
        ).addIntegerOption(option =>
            continentChoice(option, "continent of the new nation").setRequired(true)
        ).addUserOption(option =>
            option.setName(USER_ID).setDescription("user of the new nation").setRequired(true)
        ),
    async execute(interaction: CommandInteraction) {
        if (!isAdmin(interaction.member)){
            interaction.reply({content: `you are not an admin.`, ephemeral: true});
            return;
        }
        const name = interaction.options.getString(NAME_ID);
        const continent = allContinents[interaction.options.getInteger(CONTINENT_ID)];
        const ownerDiscord = interaction.options.getUser(USER_ID).id;

        await createNation({name, continent, ownerDiscord, removed: false});
        interaction.reply(`successfully created ${name}`);
    },
    category: "Edit Nation"
} as UlinaCommand;