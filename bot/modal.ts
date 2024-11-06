import * as database from "../server/database";
import {isAdmin} from "./bot";
import {ModalSubmitInteraction} from "discord.js";
import {DESCRIPTION_INPUT_ID, NAME_INPUT_ID} from "./identifiers";
import {replyOk, validateUser} from "./nation-edit";

interface Nation{
    nationId: number,
    name: string,
    ownerDiscord: string
}

export async function execute(interaction: ModalSubmitInteraction){
    const nationId = parseInt(interaction.customId);
    const nation = await database.selectNation(database.nationById(nationId),
        {nationId: true, name: true, ownerDiscord: true}
    ) as Nation;

    const valid = validateUser(nation, interaction.member);
    if (valid.err){
        interaction.reply(valid.message);
        return;
    }

    const info: database.UpdateNation = {
        description: interaction.fields.getTextInputValue(DESCRIPTION_INPUT_ID),
        name: interaction.fields.getTextInputValue(NAME_INPUT_ID),
        nationId: nation.nationId,
    }
    const updated = await database.updateNation(info, isAdmin(interaction.member))
    replyOk(interaction, updated.name);
}