import {
    CommandInteraction,
    MessageActionRow,
    Modal,
    ModalActionRowComponent,
    TextInputComponent
} from "discord.js";
import {NationAndSocials} from "../../server/database";
import {UlinaCommand} from "../bot";
import {EditType, editData, editExecute} from "../nation-edit";
import {DESCRIPTION_INPUT_ID, NAME_INPUT_ID} from "../identifiers";
import {OkVoid} from "../../shared/result";
import {SlashCommandBuilder} from "@discordjs/builders";

async function execute(interaction: CommandInteraction, nation: NationAndSocials){
    const modal = new Modal().setCustomId(nation.nationId.toString()).setTitle(`Edit ${nation.name}`);
    const nameInput = new TextInputComponent()
        .setCustomId(NAME_INPUT_ID)
        .setLabel("Name")
        .setRequired(true)
        .setValue(nation.name)
        .setStyle("SHORT");

    const descriptionInput = new TextInputComponent()
        .setCustomId(DESCRIPTION_INPUT_ID)
        .setLabel("Description")
        .setRequired(false)
        .setStyle("PARAGRAPH");

    if (nation.description !== null){
        descriptionInput.setValue(nation.description);
    }

    [nameInput, descriptionInput].forEach(input => {
        modal.addComponents(new MessageActionRow<ModalActionRowComponent>().addComponents(input));
    });
    await interaction.showModal(modal);
    return OkVoid();
}

const edit: EditType = {
    name: "name and description",
    execute,
    adminOnly: false
}

module.exports = {
    data: editData(edit, new SlashCommandBuilder()),
    execute: editExecute(edit),
    category: "Edit Nation"
} as UlinaCommand;