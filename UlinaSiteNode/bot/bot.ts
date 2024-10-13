import {Collection, CommandInteraction, GuildMember, ModalSubmitInteraction} from "discord.js";
import * as modal from "./modal";
import {APIGuildMember, RESTPostAPIApplicationCommandsJSONBody} from "discord-api-types/v9";
import {SlashCommandBuilder} from "@discordjs/builders";
import {readdirSync} from "fs";

const fs = require('node:fs');
const path = require('node:path');
const { Client, Intents } = require('discord.js');

const client = new Client({ intents: [Intents.FLAGS.GUILDS, Intents.FLAGS.GUILD_MESSAGES] });
type CommandCategory = "View Nation" | "Edit Nation" | "Time" | "Misc" | "Help";

export interface UlinaCommand{
    data: SlashCommandBuilder,
    execute: (CommandInteraction) => Promise<void>,
    category: CommandCategory
}

export const commands = new Collection<string, UlinaCommand>();
const commandsPath = path.join(__dirname, 'commands');

const commandsJson =
    readdirSync(commandsPath)
        .filter(file => file.endsWith('.js'))
        .map(file => {
            const filePath = path.join(commandsPath, file);
            const command: UlinaCommand = require(filePath);
            commands.set(command.data.name, command);
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
interface Config {
    admin: string
}
const config: Config = JSON.parse(fs.readFileSync(configPath, "utf-8"));

client.on("messageCreate", async interaction => {

    if (interaction.content.includes("ulina-admin") && interaction.member.user.id === MOE){

        config.admin = interaction.content.replace("ulina-admin", "").trim();
        fs.writeFileSync(configPath, JSON.stringify(config));

        await interaction.reply("Saved config successfully");
    }

});

client.on('interactionCreate', async interaction => {
    let action;
    if (interaction.isModalSubmit()){
        console.log(`modal ${(interaction as ModalSubmitInteraction).customId} submitted`);
        action = modal.execute;
    }
    else if (interaction.isCommand()) {
        const command = commands.get(interaction.commandName);
        if (!command) return;
        console.log(`command ${command.data.name} called`)
        action = command.execute;
    }
    else{
        return;
    }

    try {
        await action(interaction);
    } catch (error) {
        console.error(error);
        await interaction.reply({ content: 'There was an error while executing!', ephemeral: true });
    }
});

client.login(process.env.DISCORD_TOKEN).then(() => {
    client.user.setPresence({activities: [{name: "/commands", type: "LISTENING"}], status: "online"});
});

export function isAdmin(user: GuildMember | APIGuildMember): boolean{
    user = user as unknown as GuildMember;
    let isAdmin = false;
    if (user.roles != undefined){
        isAdmin = user.roles.cache.some((role) => role.id === config.admin);
    }

    return user.id === MOE || isAdmin;
}