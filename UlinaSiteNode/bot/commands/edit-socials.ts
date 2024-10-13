import {editData, editExecute, EditType, replyOk} from "../nation-edit";
import {LINK_ID, MODE_ID, PLATFORM_ID} from "../identifiers";
import {isAdmin, UlinaCommand} from "../bot";
import {CommandInteraction} from "discord.js";
import {CreateSocial, NationAndSocials, Social} from "../../server/database";
import {Err, ErrVoid, Ok, OkVoid, Result} from "../../shared/result";
import * as database from "../../server/database";
import {SlashCommandBuilder} from "@discordjs/builders";

async function execute(interaction: CommandInteraction, nation: NationAndSocials) {
    const mode = modes[interaction.options.getInteger(MODE_ID)];
    const platform = interaction.options.getString(PLATFORM_ID).trim();
    let link = interaction.options.getString(LINK_ID);

    if (mode.link){
        if (link === null){
            return ErrVoid(`a link is required to ${mode.name} a social`);
        }
        else if (!link.includes("https://")){
            return ErrVoid(`${link} is an invalid link`);
        }
        link = link.trim();
    }

    const result = mode.action(nation.socials, platform, link);
    if (result.err){
        return ErrVoid(result.message);
    }
    else{
        const updated = await database.updateNation({nationId: nation.nationId, socials: result.value}, isAdmin(interaction.member));
        replyOk(interaction, updated.name);
        return OkVoid();
    }

}

function present(socials: Social[], platform: string): boolean{
    return socials.some(social => equal(social, platform));
}

function equal(social: Social, platform: string): boolean{
    return social.platform.toLowerCase() === platform.toLowerCase();
}

function getIndex(socials: Social[], platform: string): Result<number>{
    const index = socials.findIndex(social => equal(social, platform));
    if (index < 0){
        return Err(`no social found for "${platform}"`);
    }
    return Ok(index);
}

function createSocial(socials: Social[], platform: string, link: string): Result<CreateSocial[]>{
    if (present(socials, platform)){
        return Err(`the social for "${platform}" is already assigned`);
    }
    const updated = (socials as CreateSocial[]);
    updated.push({link, platform});

    return Ok(updated);
}

function updateSocial(socials: Social[], platform: string, link: string): Result<CreateSocial[]>{
    return getIndex(socials, platform)
        .chain(index => {
        socials[index].link = link;
        return Ok(socials as CreateSocial[])
    });
}

function deleteSocial(socials: Social[], platform: string, _: string): Result<CreateSocial[]>{
    return getIndex(socials, platform)
        .chain(index => {
            socials.splice(index, 1);
            return Ok(socials as CreateSocial[]);
        });
}

interface editSocial{
    action: (socials: Social[], platform: string, link: string) => Result<CreateSocial[]>,
    name: string
    link: boolean
}

const modes: editSocial[] = [
    {action: createSocial, name: "create", link: true},
    {action: updateSocial, name: "update", link: true},
    {action: deleteSocial, name: "delete", link: false},
]

const edit: EditType = {
    name: "socials",
    execute,
    adminOnly: false
}

module.exports = {
    data: editData(edit, new SlashCommandBuilder().addStringOption(option =>
        option.setName(PLATFORM_ID)
            .setDescription("the platform to edit")
            .setRequired(true)
    ).addIntegerOption(option => {
        option.setName(MODE_ID).setDescription(`"action to do - specify the "link" option for "create" or "update"`).setRequired(true)

        modes.forEach((mode, index) => {
            option.addChoices({name: mode.name, value: index});
        });

        return option;
    }).addStringOption(option =>
        option.setName(LINK_ID)
            .setDescription("the link of the social - required for creation or updating")
    )),
    execute: editExecute(edit),
    category: "Edit Nation"
} as UlinaCommand;