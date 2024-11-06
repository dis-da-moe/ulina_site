"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.isAdmin = exports.commands = void 0;
const discord_js_1 = require("discord.js");
const modal = require("./modal");
const fs_1 = require("fs");
const fs = require('node:fs');
const path = require('node:path');
const { Client, Intents } = require('discord.js');
const client = new Client({ intents: [Intents.FLAGS.GUILDS, Intents.FLAGS.GUILD_MESSAGES] });
exports.commands = new discord_js_1.Collection();
const commandsPath = path.join(__dirname, 'commands');
const commandsJson = (0, fs_1.readdirSync)(commandsPath)
    .filter(file => file.endsWith('.js'))
    .map(file => {
    const filePath = path.join(commandsPath, file);
    const command = require(filePath);
    exports.commands.set(command.data.name, command);
    return command.data.toJSON();
});
const { REST } = require('@discordjs/rest');
const { Routes } = require('discord-api-types/v9');
const rest = new REST({ version: '9' }).setToken(process.env.DISCORD_TOKEN);
rest.put(Routes.applicationCommands(process.env.CLIENT_ID), { body: commandsJson })
    .then(() => console.log('Successfully registered application commands.'))
    .catch(console.error);
client.once('ready', () => {
    console.log('Ready!');
});
const MOE = "368673056899596290";
const configPath = "./data/config.json";
const config = JSON.parse(fs.readFileSync(configPath, "utf-8"));
client.on("messageCreate", async (interaction) => {
    if (interaction.content.includes("ulina-admin") && interaction.member.user.id === MOE) {
        config.admin = interaction.content.replace("ulina-admin", "").trim();
        fs.writeFileSync(configPath, JSON.stringify(config));
        await interaction.reply("Saved config successfully");
    }
});
client.on('interactionCreate', async (interaction) => {
    let action;
    if (interaction.isModalSubmit()) {
        console.log(`modal ${interaction.customId} submitted`);
        action = modal.execute;
    }
    else if (interaction.isCommand()) {
        const command = exports.commands.get(interaction.commandName);
        if (!command)
            return;
        console.log(`command ${command.data.name} called`);
        action = command.execute;
    }
    else {
        return;
    }
    try {
        await action(interaction);
    }
    catch (error) {
        console.error(error);
        await interaction.reply({ content: 'There was an error while executing!', ephemeral: true });
    }
});
client.login(process.env.DISCORD_TOKEN).then(() => {
    client.user.setPresence({ activities: [{ name: "/commands", type: "LISTENING" }], status: "online" });
});
function isAdmin(user) {
    user = user;
    let isAdmin = false;
    if (user.roles != undefined) {
        isAdmin = user.roles.cache.some((role) => role.id === config.admin);
    }
    return user.id === MOE || isAdmin;
}
exports.isAdmin = isAdmin;
//# sourceMappingURL=bot.js.map