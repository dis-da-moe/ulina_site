import * as fs from "fs";
import {SlashCommandBuilder} from "@discordjs/builders";
import {CommandInteraction, MessageEmbed} from "discord.js";
import {UlinaCommand} from "../bot";
import {GUIDE_ID} from "../identifiers";
import {capitalise} from "../../server/utilities";
import {colorEmbed} from "../nation-edit";

interface Guide{
    name: string,
    image: string,
    title: string,
    link: string,
    description: string,
    includes: string[]
}

const guides: Guide[] = JSON.parse(fs.readFileSync("./data/guide.json", "utf-8")).guides;

module.exports = {
    data: new SlashCommandBuilder()
        .setName('guides')
        .setDescription('view guides for getting acquainted with Ulina')
        .addIntegerOption(option => {
            option.setName(GUIDE_ID).setDescription("the guide to view").setRequired(true);
            guides.forEach((guide, index) => option
                .addChoices({name: capitalise(guide.name), value: index}));
            return option;
        }),
    async execute(interaction: CommandInteraction) {
        const guide = guides[interaction.options.getInteger(GUIDE_ID)];
        const embed = colorEmbed()
            .setTitle(guide.title)
            .setURL(guide.link)
            .setDescription(`${guide.link}\n${guide.description}`)
            .setImage(guide.image).addField("This includes: ", "\u200B");
        guide.includes.forEach(include => {
            embed.addField(`• **${capitalise(include)}**`, "\u200B",true,);
        });
        interaction.reply({embeds: [embed]});
    },
    category: "Help"
} as UlinaCommand;