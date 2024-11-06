import {CommandInteraction, GuildMember, MessageEmbed, ModalSubmitInteraction} from "discord.js";
import * as database from "../server/database";
import {allContinents, hasDiscordId, NationAndSocials} from "../server/database";
import {isAdmin} from "./bot";
import {SlashCommandBuilder, SlashCommandIntegerOption} from "@discordjs/builders";
import {CONTINENT_ID, NAME_ID} from "./identifiers";
import {Err, Ok, Result} from "../shared/result";
import {APIGuildMember} from "discord-api-types/v9";

export interface EditType{
    name: string,
    execute: (CommandInteraction, NationAndSocials) => Promise<Result<void>>,
    adminOnly: boolean
}

export function replyOk(interaction: CommandInteraction|ModalSubmitInteraction, name: string){
    interaction.reply(`${name} successfully edited`);
}

export function validateUser<NationType extends hasDiscordId>(nation: NationType, member: GuildMember | APIGuildMember):
    Result<NationType>{

    member = member as unknown as GuildMember;
    const isOwner = nation.ownerDiscord !== "" && nation.ownerDiscord === member.user.id;

    if (!isOwner && !isAdmin(member)){
        return Err(`this nation does not belong to you - if this is a mistake contact the moderators`);
    }
    else{
        return Ok(nation);
    }
}

export async function nationByUser(member: GuildMember | APIGuildMember):
    Promise<Result<NationAndSocials>>{

    const results = await database.selectNations({ownerDiscord: member.user.id, removed: false}, database.nationAndSocials);

    if (results.length < 1){
        return Err(
            `no nation was found linked to your account - ${isAdmin(member) ? "try specifying a name" : "contact moderators if this is a mistake"}`
        );
    }
    return Ok(results[0] as NationAndSocials);
}

export function colorEmbed(){
    return new MessageEmbed().setColor("#0f4272");
}

export function editData(edit: EditType, command: Omit<SlashCommandBuilder, "addSubcommand" | "addSubcommandGroup">): Omit<SlashCommandBuilder, "addSubcommand" | "addSubcommandGroup">{
    return command
        .setName("edit-" + edit.name.replace(" and ", " ").replaceAll(' ', '-'))
        .setDescription(edit.adminOnly ? `admin only - change the ${edit.name} of a nation` : `change the ${edit.name} of your ulina nation`)
        .addStringOption(input =>
            input.setName(NAME_ID)
                .setDescription(`${edit.adminOnly ? "" : "admin only -"} name of the ulina country to edit`)
                .setRequired(edit.adminOnly)
        );
}

export function editExecute(edit: EditType): (interaction: CommandInteraction) => Promise<void>{
    return async (interaction: CommandInteraction) => {
        const name = interaction.options.getString(NAME_ID);

        if (edit.adminOnly && !isAdmin(interaction.member)) {
            interaction.reply({content: `only a moderator can change the ${edit.name} of a nation - contact one if this is desired`, ephemeral: true});
            return;
        }
        const nation: Result<NationAndSocials> =
            name === null
            ? await nationByUser(interaction.member)
            : (await database.nationByName<NationAndSocials>(name, database.nationAndSocials))
            .chain(nation =>
                validateUser(nation, interaction.member)
            );

        const result = await nation.asyncChain(nation =>
                edit.execute(interaction, nation)
            );

        if (result.err){
            if (interaction.replied){
                interaction.editReply(result.message);
            }
            else{
                interaction.reply({content: result.message, ephemeral: true});
            }
        }
    };
}

export function continentChoice(option: SlashCommandIntegerOption, description: string): SlashCommandIntegerOption{
    option = option.setName(CONTINENT_ID).setDescription(description);
    allContinents.forEach((continent, index) => option.addChoices({name: continent, value: index}));
    return option;
}