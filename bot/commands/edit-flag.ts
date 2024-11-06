import {editData, editExecute, EditType, replyOk} from "../nation-edit";
import {FLAG_ID} from "../identifiers";
import {isAdmin, UlinaCommand} from "../bot";
import {CommandInteraction} from "discord.js";
import * as database from "../../server/database";
import {NationAndSocials, UpdateNation} from "../../server/database";
import {ErrVoid, OkVoid} from "../../shared/result";
import {matchesImageExtension} from "../../server/utilities";
import axios from "axios";
import {SlashCommandBuilder} from "@discordjs/builders";


async function execute(interaction: CommandInteraction, nation: NationAndSocials){
    const attachment = interaction.options.getAttachment(FLAG_ID);
    const extension = matchesImageExtension(attachment.name);

    if (extension.err){
        return ErrVoid(`${attachment.name} is not an image file`);
    }
    await interaction.reply({content: `updating...`, ephemeral: true});
    const response =  await axios.get(attachment.url, { responseType: 'arraybuffer' });
    const info: UpdateNation = {
        nationId: nation.nationId,
        flag: {
            file: Buffer.from(response.data),
            fileExtension: extension.value
        }
    };
    const updated = await database.updateNation(info, isAdmin(interaction.member));
    interaction.followUp(`edited flag of ${updated.name} successfully`);
    return OkVoid();
}

const edit: EditType = {
    name: "flag",
    execute,
    adminOnly: false
}

module.exports = {
    data: editData(edit, new SlashCommandBuilder().addAttachmentOption(option =>
        option.setName(FLAG_ID)
            .setDescription("the new flag of the country")
            .setRequired(true)
    )),
    execute: editExecute(edit),
    category: "Edit Nation"
} as UlinaCommand;