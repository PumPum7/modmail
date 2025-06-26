import { REST } from '@discordjs/rest';
import { Routes } from 'discord-api-types/v9';
import { SlashCommandBuilder } from '@discordjs/builders';
import 'dotenv/config';

const commands = [
    new SlashCommandBuilder().setName('message').setDescription('Send a message to a user')
        .addStringOption(option => option.setName('user').setDescription('The user to message (ID)').setRequired(true))
        .addStringOption(option => option.setName('message').setDescription('The message to send').setRequired(true)),
    new SlashCommandBuilder().setName('close').setDescription('Close the current thread'),
    new SlashCommandBuilder().setName('macro').setDescription('Manage macros')
        .addSubcommand(subcommand =>
            subcommand.setName('create').setDescription('Create a new macro')
                .addStringOption(option => option.setName('name').setDescription('The name of the macro').setRequired(true))
                .addStringOption(option => option.setName('content').setDescription('The content of the macro').setRequired(true)))
        .addSubcommand(subcommand =>
            subcommand.setName('send').setDescription('Send a macro')
                .addStringOption(option => option.setName('name').setDescription('The name of the macro').setRequired(true)))
        .addSubcommand(subcommand =>
            subcommand.setName('delete').setDescription('Delete a macro')
                .addStringOption(option => option.setName('name').setDescription('The name of the macro to delete').setRequired(true)))
].map(command => command.toJSON());

const rest = new REST({ version: '9' }).setToken(process.env.DISCORD_BOT_TOKEN!);

(async () => {
    try {
        console.log('Started refreshing application (/) commands.');

        await rest.put(
            Routes.applicationGuildCommands(process.env.PUBLIC_DISCORD_CLIENT_ID!, process.env.DISCORD_SERVER_ID!),
            { body: commands },
        );

        console.log('Successfully reloaded application (/) commands.');
    } catch (error) {
        console.error(error);
    }
})();
