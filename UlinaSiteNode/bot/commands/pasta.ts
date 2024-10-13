import {SlashCommandBuilder} from "@discordjs/builders";
import fs = require('fs');
import {CommandInteraction} from "discord.js";
import {UlinaCommand} from "../bot";

interface Pasta{
    name: string,
    content: string
}
const pastas: Pasta[] = JSON.parse(fs.readFileSync("./data/pasta.json", "utf-8")).pastas;

module.exports = {
    data: new SlashCommandBuilder()
        .setName('pasta')
        .setDescription('sends the ulina copy-pasta of your choice')
        .addIntegerOption(input => {
                const option = input.setName("pasta").setRequired(true)
                    .setDescription("the selected pasta");

                pastas.forEach((pasta, index) => {
                    option.addChoices({name: pasta.name, value: index})
                });
                return option;
            }
        ),
    async execute(interaction: CommandInteraction) {
        const pasta = pastas[interaction.options.getInteger("pasta")];
        await interaction.reply(pasta.content);
    },
    category: "Misc"
} as UlinaCommand;