import {CommandInteraction} from "discord.js";
import {UlinaCommand} from "../bot";
import {NationAndSocials, updateNation} from "../../server/database";
import {editData, editExecute, EditType} from "../nation-edit";
import {OkVoid} from "../../shared/result";
import {SlashCommandBuilder} from "@discordjs/builders";

async function execute(interaction: CommandInteraction, nation: NationAndSocials) {
    await updateNation({nationId: nation.nationId, removed: true});
    interaction.reply(`successfully removed ${nation.name}`);
    return OkVoid();
}

const edit: EditType = {
    adminOnly: true,
    execute,
    name: "remove nation"
}

module.exports = {
    data: editData(edit, new SlashCommandBuilder())
        .setDescription('admin only - remove a nation'),
    execute: editExecute(edit),
    category: "Edit Nation"
} as UlinaCommand;