"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.execute = void 0;
const database = require("../server/database");
const bot_1 = require("./bot");
const identifiers_1 = require("./identifiers");
const nation_edit_1 = require("./nation-edit");
async function execute(interaction) {
    const nationId = parseInt(interaction.customId);
    const nation = await database.selectNation(database.nationById(nationId), { nationId: true, name: true, ownerDiscord: true });
    const valid = (0, nation_edit_1.validateUser)(nation, interaction.member);
    if (valid.err) {
        interaction.reply(valid.message);
        return;
    }
    const info = {
        description: interaction.fields.getTextInputValue(identifiers_1.DESCRIPTION_INPUT_ID),
        name: interaction.fields.getTextInputValue(identifiers_1.NAME_INPUT_ID),
        nationId: nation.nationId,
    };
    const updated = await database.updateNation(info, (0, bot_1.isAdmin)(interaction.member));
    (0, nation_edit_1.replyOk)(interaction, updated.name);
}
exports.execute = execute;
//# sourceMappingURL=modal.js.map