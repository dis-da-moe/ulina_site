import {NationAndSocials} from "../server/database";
import {MessageEmbed} from "discord.js";
import * as database from "../server/database";
import {colorEmbed} from "./nation-edit";

export function displayUser(nation: NationAndSocials){
    return nation.ownerDiscord === "" ? "`not assigned`" : `<@${nation.ownerDiscord}>`
}

export async function nationEmbed(nation: NationAndSocials){
    const embed = colorEmbed().setTitle(`${nation.name}`);

    if (nation.description !== null){
        embed.setDescription(nation.description);
    }
    embed.addField("Continent", nation.continent.name);

    for (const social of nation.socials){
        embed.addField(social.platform, social.link);
    }

    if (nation.currentFlagId !== null){
        const flag = await database.currentFlag({nationId: nation.nationId});
        const path = `https://www.ulinaworld.com${flag}`;
        embed.setImage(path);
    }

    embed.addField("Owner", displayUser(nation));

    embed.setFooter({text: "contact moderators if this content breaks the rules"});

    return embed;
}