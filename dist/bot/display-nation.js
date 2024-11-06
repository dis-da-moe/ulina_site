"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.nationEmbed = exports.displayUser = void 0;
const database = require("../server/database");
const nation_edit_1 = require("./nation-edit");
function displayUser(nation) {
    return nation.ownerDiscord === "" ? "`not assigned`" : `<@${nation.ownerDiscord}>`;
}
exports.displayUser = displayUser;
async function nationEmbed(nation) {
    const embed = (0, nation_edit_1.colorEmbed)().setTitle(`${nation.name}`);
    if (nation.description !== null) {
        embed.setDescription(nation.description);
    }
    embed.addField("Continent", nation.continent.name);
    for (const social of nation.socials) {
        embed.addField(social.platform, social.link);
    }
    if (nation.currentFlagId !== null) {
        const flag = await database.currentFlag({ nationId: nation.nationId });
        const path = `https://www.ulinaworld.com${flag}`;
        embed.setImage(path);
    }
    embed.addField("Owner", displayUser(nation));
    embed.setFooter({ text: "contact moderators if this content breaks the rules" });
    return embed;
}
exports.nationEmbed = nationEmbed;
//# sourceMappingURL=display-nation.js.map