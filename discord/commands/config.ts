import { ChatInputCommandInteraction, MessageFlagsBitField, PermissionFlagsBits } from 'discord.js';
import { getGuildConfig, createGuildConfig, updateGuildConfig } from '../api.js';

export async function handleConfigCommand(interaction: ChatInputCommandInteraction) {
	// Check if user has administrator permissions
	if (!interaction.memberPermissions?.has(PermissionFlagsBits.Administrator)) {
		await interaction.reply({
			content: '❌ You need Administrator permissions to use this command.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
		return;
	}

	const subcommand = interaction.options.getSubcommand();
	const guildId = interaction.guildId!;

	try {
		switch (subcommand) {
			case 'show':
				await handleConfigShow(interaction, guildId);
				break;
			case 'set':
				await handleConfigSet(interaction, guildId);
				break;
			case 'add':
				await handleConfigAdd(interaction, guildId);
				break;
			case 'remove':
				await handleConfigRemove(interaction, guildId);
				break;
			case 'reset':
				await handleConfigReset(interaction, guildId);
				break;
			default:
				await interaction.reply({
					content: '❌ Invalid subcommand.',
					flags: MessageFlagsBitField.Flags.Ephemeral,
				});
				break;
		}
	} catch (error) {
		console.error('Error handling config command:', error);
		await interaction.reply({
			content: '❌ Failed to execute config command.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	}
}

async function handleConfigShow(interaction: ChatInputCommandInteraction, guildId: string) {
	try {
		const config = await getGuildConfig(guildId);

		const configText = `**Current Configuration:**
**Modmail Category:** ${config.modmail_category_id ? `<#${config.modmail_category_id}>` : 'Not set'}
**Log Channel:** ${config.log_channel_id ? `<#${config.log_channel_id}>` : 'Not set'}
**Randomize Names:** ${config.randomize_names ? 'Enabled' : 'Disabled'}
**Auto Close Hours:** ${config.auto_close_hours || 'Not set'}
**Welcome Message:** ${config.welcome_message || 'Default'}
**Moderator Roles:** ${config.moderator_role_ids?.length ? config.moderator_role_ids.map((id: string) => `<@&${id}>`).join(', ') : 'None'}
**Blocked Words:** ${config.blocked_words?.length ? config.blocked_words.join(', ') : 'None'}`;

		await interaction.reply({
			content: configText,
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	} catch (error) {
		// Config doesn't exist, create default one
		const defaultConfig = {
			guild_id: guildId,
			randomize_names: false,
		};

		await createGuildConfig(guildId, defaultConfig);
		await interaction.reply({
			content:
				'✅ Created default configuration for this server. Use `/config show` to view settings.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	}
}

async function handleConfigSet(interaction: ChatInputCommandInteraction, guildId: string) {
	const setting = interaction.options.getString('setting', true);
	const value = interaction.options.getString('value', true);

	let updateData: any = {};

	switch (setting) {
		case 'modmail-category':
			const categoryChannel = interaction.options.getChannel('channel');
			if (!categoryChannel || categoryChannel.type !== 4) {
				// CategoryChannel
				await interaction.reply({
					content: '❌ Please provide a valid category channel.',
					flags: MessageFlagsBitField.Flags.Ephemeral,
				});
				return;
			}
			updateData.modmail_category_id = categoryChannel.id;
			break;
		case 'log-channel':
			const logChannel = interaction.options.getChannel('channel');
			if (!logChannel || logChannel.type !== 0) {
				await interaction.reply({
					content: '❌ Please provide a valid text channel.',
					flags: MessageFlagsBitField.Flags.Ephemeral,
				});
				return;
			}
			updateData.log_channel_id = logChannel.id;
			break;
		case 'randomize-names':
			const randomize = value.toLowerCase() === 'true';
			updateData.randomize_names = randomize;
			break;
		case 'auto-close-hours':
			const hours = parseInt(value);
			if (isNaN(hours) || hours < 1) {
				await interaction.reply({
					content: '❌ Please provide a valid number of hours (minimum 1).',
					flags: MessageFlagsBitField.Flags.Ephemeral,
				});
				return;
			}
			updateData.auto_close_hours = hours;
			break;
		case 'welcome-message':
			updateData.welcome_message = value;
			break;
		default:
			await interaction.reply({
				content:
					'❌ Invalid setting. Valid settings: modmail-category, log-channel, randomize-names, auto-close-hours, welcome-message',
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
			return;
	}

	await updateGuildConfig(guildId, updateData);
	await interaction.reply({
		content: `✅ Updated ${setting} successfully.`,
		flags: MessageFlagsBitField.Flags.Ephemeral,
	});
}

async function handleConfigAdd(interaction: ChatInputCommandInteraction, guildId: string) {
	const type = interaction.options.getString('type', true);
	const value = interaction.options.getString('value', true);

	const config = await getGuildConfig(guildId);
	let updateData: any = {};

	switch (type) {
		case 'moderator-role':
			const role = interaction.options.getRole('role');
			if (!role) {
				await interaction.reply({
					content: '❌ Please provide a valid role.',
					flags: MessageFlagsBitField.Flags.Ephemeral,
				});
				return;
			}
			const currentRoles = config.moderator_role_ids || [];
			if (!currentRoles.includes(role.id)) {
				updateData.moderator_role_ids = [...currentRoles, role.id];
			}
			break;
		case 'blocked-word':
			const currentWords = config.blocked_words || [];
			if (!currentWords.includes(value)) {
				updateData.blocked_words = [...currentWords, value];
			}
			break;
		default:
			await interaction.reply({
				content: '❌ Invalid type. Valid types: moderator-role, blocked-word',
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
			return;
	}

	await updateGuildConfig(guildId, updateData);
	await interaction.reply({
		content: `✅ Added ${type} successfully.`,
		flags: MessageFlagsBitField.Flags.Ephemeral,
	});
}

async function handleConfigRemove(interaction: ChatInputCommandInteraction, guildId: string) {
	const type = interaction.options.getString('type', true);
	const value = interaction.options.getString('value', true);

	const config = await getGuildConfig(guildId);
	let updateData: any = {};

	switch (type) {
		case 'moderator-role':
			const role = interaction.options.getRole('role');
			if (!role) {
				await interaction.reply({
					content: '❌ Please provide a valid role.',
					flags: MessageFlagsBitField.Flags.Ephemeral,
				});
				return;
			}
			const currentRoles = config.moderator_role_ids || [];
			updateData.moderator_role_ids = currentRoles.filter((id: string) => id !== role.id);
			break;
		case 'blocked-word':
			const currentWords = config.blocked_words || [];
			updateData.blocked_words = currentWords.filter((word: string) => word !== value);
			break;
		default:
			await interaction.reply({
				content: '❌ Invalid type. Valid types: moderator-role, blocked-word',
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
			return;
	}

	await updateGuildConfig(guildId, updateData);
	await interaction.reply({
		content: `✅ Removed ${type} successfully.`,
		flags: MessageFlagsBitField.Flags.Ephemeral,
	});
}

async function handleConfigReset(interaction: ChatInputCommandInteraction, guildId: string) {
	const defaultConfig = {
		modmail_category_id: null,
		log_channel_id: null,
		randomize_names: false,
		auto_close_hours: null,
		welcome_message: null,
		moderator_role_ids: [],
		blocked_words: [],
	};

	await updateGuildConfig(guildId, defaultConfig);
	await interaction.reply({
		content: '✅ Configuration reset to defaults.',
		flags: MessageFlagsBitField.Flags.Ephemeral,
	});
}
