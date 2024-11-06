import {SlashCommandBuilder} from '@discordjs/builders';
import {CommandInteraction, MessageEmbed} from "discord.js";
import {UlinaCommand} from "../bot";
import {commands} from "../bot";
import {colorEmbed} from "../nation-edit";

interface DisplayCommand{
    name: string,
    value: string
}

module.exports = {
    data: new SlashCommandBuilder()
        .setName('commands')
        .setDescription('list all commands'),
    async execute(interaction: CommandInteraction) {
        const categories = new Map<string, DisplayCommand[]>();
        for (const command of commands){
            const category = categories.get(command[1].category);

            const displayCommand = {name: command[0], value: command[1].data.description};

            if (category === undefined){
                categories.set(command[1].category, [displayCommand]);
            }
            else{
                category.push(displayCommand);
            }
        }

        const embed = colorEmbed().setTitle("Moley Commands")
            .setDescription("commands and their descriptions");

        categories.forEach((displayCommands, category) => {
            embed.addField(`__${category}__`, "\u200B");
            for (const command of displayCommands){
                embed.addField(command.name, command.value, true);
            }
        });

        return interaction.reply({embeds: [embed]});
    },
    category: "Help"
} as UlinaCommand;