import { REST } from '@discordjs/rest';
import { Routes } from 'discord-api-types/v9';
import { SlashCommandBuilder } from '@discordjs/builders';
import 'dotenv/config';

const commands = [
	new SlashCommandBuilder()
		.setName('message')
		.setDescription('Send a message to a user')
		.addUserOption((option) =>
			option.setName('user').setDescription('The user to message').setRequired(true)
		)
		.addStringOption((option) =>
			option.setName('message').setDescription('The message to send').setRequired(true)
		),
	new SlashCommandBuilder().setName('close').setDescription('Close the current thread'),
	new SlashCommandBuilder().setName('delete').setDescription('Delete the current thread'),
	new SlashCommandBuilder()
		.setName('note')
		.setDescription('Add an internal moderator note to the thread')
		.addStringOption((option) =>
			option.setName('content').setDescription('The note content').setRequired(true)
		),
	new SlashCommandBuilder()
		.setName('block')
		.setDescription('Block a user from creating modmail threads')
		.addUserOption((option) =>
			option.setName('user').setDescription('The user to block').setRequired(true)
		)
		.addStringOption((option) =>
			option.setName('reason').setDescription('Reason for blocking').setRequired(false)
		),
	new SlashCommandBuilder()
		.setName('unblock')
		.setDescription('Unblock a user from creating modmail threads')
		.addUserOption((option) =>
			option.setName('user').setDescription('The user to unblock').setRequired(true)
		),
	new SlashCommandBuilder()
		.setName('urgency')
		.setDescription('Change the urgency level of the current thread')
		.addStringOption((option) =>
			option
				.setName('level')
				.setDescription('The urgency level')
				.setRequired(true)
				.addChoices(
					{ name: 'Low', value: 'Low' },
					{ name: 'Medium', value: 'Medium' },
					{ name: 'High', value: 'High' },
					{ name: 'Urgent', value: 'Urgent' }
				)
		),
	new SlashCommandBuilder()
		.setName('macro')
		.setDescription('Manage macros')
		.addSubcommand((subcommand) =>
			subcommand
				.setName('create')
				.setDescription('Create a new macro')
				.addStringOption((option) =>
					option.setName('name').setDescription('The name of the macro').setRequired(true)
				)
				.addStringOption((option) =>
					option.setName('content').setDescription('The content of the macro').setRequired(true)
				)
		)
		.addSubcommand((subcommand) =>
			subcommand
				.setName('send')
				.setDescription('Send a macro')
				.addStringOption((option) =>
					option.setName('name').setDescription('The name of the macro').setRequired(true)
				)
		)
		.addSubcommand((subcommand) =>
			subcommand
				.setName('delete')
				.setDescription('Delete a macro')
				.addStringOption((option) =>
					option.setName('name').setDescription('The name of the macro to delete').setRequired(true)
				)
		)
		.addSubcommand((subcommand) => subcommand.setName('list').setDescription('List all macros'))
		.addSubcommand((subcommand) =>
			subcommand
				.setName('edit')
				.setDescription('Edit a macro')
				.addStringOption((option) =>
					option.setName('name').setDescription('The name of the macro').setRequired(true)
				)
				.addStringOption((option) =>
					option.setName('content').setDescription('The new content of the macro').setRequired(true)
				)
		),
	new SlashCommandBuilder()
		.setName('config')
		.setDescription('Configure the modmail system')
		.addSubcommand((sub) => sub.setName('show').setDescription('Show the current configuration'))
		.addSubcommand((sub) =>
			sub
				.setName('set')
				.setDescription('Set a configuration option')
				.addStringOption((opt) =>
					opt
						.setName('setting')
						.setDescription(
							'(modmail-category, log-channel, randomize-names, auto-close-hours, welcome-message)'
						)
						.setRequired(true)
				)
				.addStringOption((opt) =>
					opt.setName('value').setDescription('Value for the setting').setRequired(true)
				)
				.addChannelOption((opt) =>
					opt
						.setName('channel')
						.setDescription('Channel (for modmail-category or log-channel)')
						.setRequired(false)
				)
		)
		.addSubcommand((sub) =>
			sub
				.setName('add')
				.setDescription('Add a moderator role or blocked word')
				.addStringOption((opt) =>
					opt
						.setName('type')
						.setDescription('Type to add (moderator-role, blocked-word)')
						.setRequired(true)
				)
				.addStringOption((opt) =>
					opt.setName('value').setDescription('Value to add').setRequired(true)
				)
				.addRoleOption((opt) =>
					opt.setName('role').setDescription('Role (for moderator-role)').setRequired(false)
				)
		)
		.addSubcommand((sub) =>
			sub
				.setName('remove')
				.setDescription('Remove a moderator role or blocked word')
				.addStringOption((opt) =>
					opt.setName('type').setDescription('(moderator-role, blocked-word)').setRequired(true)
				)
				.addStringOption((opt) =>
					opt.setName('value').setDescription('Value to remove').setRequired(true)
				)
				.addRoleOption((opt) =>
					opt.setName('role').setDescription('Role (for moderator-role)').setRequired(false)
				)
		)
		.addSubcommand((sub) =>
			sub.setName('reset').setDescription('Reset all configuration to defaults')
		),
].map((command) => command.toJSON());

const rest = new REST({ version: '10' }).setToken(process.env.DISCORD_BOT_TOKEN!);

(async () => {
	try {
		console.log('Started refreshing application (/) commands.');

		if (!process.env.PUBLIC_DISCORD_CLIENT_ID || !process.env.PUBLIC_DISCORD_SERVER_ID) {
			throw new Error('PUBLIC_DISCORD_CLIENT_ID and PUBLIC_DISCORD_SERVER_ID must be set');
		}

		await rest.put(
			Routes.applicationGuildCommands(
				process.env.PUBLIC_DISCORD_CLIENT_ID!,
				process.env.PUBLIC_DISCORD_SERVER_ID!
			),
			{ body: commands }
		);

		console.log('Successfully reloaded application (/) commands.');
	} catch (error) {
		console.error(error);
	}
})();
