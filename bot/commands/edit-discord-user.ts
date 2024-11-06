import {UlinaCommand} from "../bot";
import * as database from "../../server/database";
import {CommandInteraction, User} from "discord.js";
import {NationAndSocials} from "../../server/database";
import {editData, editExecute, EditType, replyOk} from "../nation-edit";
import {USER_ID} from "../identifiers";
import {OkVoid} from "../../shared/result";
import {SlashCommandBuilder} from "@discordjs/builders";

async function execute(interaction: CommandInteraction, nation: NationAndSocials){
    const user: User = interaction.options.getUser(USER_ID);

    const info: database.UpdateNation = {
        nationId: nation.nationId,
        ownerDiscord: user.id
    }

    const updated = await database.updateNation(info, true);
    replyOk(interaction, updated.name);
    return OkVoid();
}

const edit: EditType = {
    name: "discord user",
    execute,
    adminOnly: true
}

module.exports = {
    data: editData(edit,new SlashCommandBuilder())
        .addUserOption(input =>
            input.setName(USER_ID)
                .setDescription("the user that owns this nation")
                .setRequired(true)
        ),
    execute: editExecute(edit),
    category: "Edit Nation"
} as UlinaCommand;