import { ChatInputCommandInteraction, Client, MessageFlagsBitField } from 'discord.js';
import {
	createMacro,
	getMacroByName,
	deleteMacro,
	getMacros,
	editMacro,
	getThreadByChannelId,
	addMessageToThread,
} from '../api.js';
import { createModeratorMessageEmbed, createConfirmationEmbed } from '../utils.js';

export async function handleMacroCommand(interaction: ChatInputCommandInteraction, client: Client) {
	const subcommand = interaction.options.getSubcommand();
	const guildId = interaction.guildId!;

	switch (subcommand) {
		case 'create':
			await handleMacroCreate(interaction, guildId);
			break;
		case 'send':
			await handleMacroSend(interaction, client, guildId);
			break;
		case 'delete':
			await handleMacroDelete(interaction, guildId);
			break;
		case 'list':
			await handleMacroList(interaction, guildId);
			break;
		case 'edit':
			await handleMacroEdit(interaction, guildId);
			break;
		default:
			await interaction.reply({
				content: '❌ Invalid subcommand.',
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
			break;
	}
}

async function handleMacroCreate(interaction: ChatInputCommandInteraction, guildId: string) {
	const name = interaction.options.getString('name', true);
	const content = interaction.options.getString('content', true);

	try {
		await createMacro(name, content, guildId);
		await interaction.reply({
			content: `✅ Macro "${name}" created successfully.`,
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	} catch (error) {
		await interaction.reply({
			content: `❌ Failed to create macro. It may already exist.`,
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	}
}

async function handleMacroSend(
	interaction: ChatInputCommandInteraction,
	client: Client,
	guildId: string
) {
	const macroName = interaction.options.getString('name', true);
	const macro = await getMacroByName(macroName, guildId);

	if (!macro) {
		await interaction.reply({
			content: `❌ Macro "${macroName}" not found.`,
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
		return;
	}

	const thread = await getThreadByChannelId(interaction.channelId, guildId);

	if (!thread) {
		await interaction.reply({
			content: '❌ This command can only be used in a modmail thread.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
		return;
	}

	try {
		// Send macro content to user
		const user = await client.users.fetch(thread.user_id);
		const embed = createModeratorMessageEmbed(macro.content);
		await user.send({ embeds: [embed] });

		// Add to thread
		await addMessageToThread(
			thread.id,
			guildId,
			interaction.user.id,
			interaction.user.tag,
			`[MACRO: ${macroName}] ${macro.content}`
		);

		// Confirm in channel
		const confirmEmbed = createConfirmationEmbed(
			user,
			macro.content,
			`Macro "${macroName}" sent to`
		);

		await interaction.reply({ embeds: [confirmEmbed] });
	} catch (error) {
		console.error('Error sending macro:', error);
		await interaction.reply({
			content: '❌ Failed to send macro.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	}
}

async function handleMacroDelete(interaction: ChatInputCommandInteraction, guildId: string) {
	const deleteNameParam = interaction.options.getString('name', true);

	try {
		const result = await deleteMacro(deleteNameParam, guildId);

		if (result.success) {
			await interaction.reply({
				content: `✅ Macro "${deleteNameParam}" deleted successfully.`,
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
		} else {
			await interaction.reply({
				content: `❌ ${result.message}`,
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
		}
	} catch (error) {
		console.error('Error deleting macro:', error);
		await interaction.reply({
			content: '❌ Failed to delete macro.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	}
}

async function handleMacroList(interaction: ChatInputCommandInteraction, guildId: string) {
	try {
		const macros = await getMacros(guildId);
		await interaction.reply({
			content: `✅ Macros: ${macros.map((m) => m.name).join(', ')}`,
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	} catch (error) {
		console.error('Error listing macros:', error);
		await interaction.reply({
			content: '❌ Failed to list macros.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	}
}

async function handleMacroEdit(interaction: ChatInputCommandInteraction, guildId: string) {
	const editNameParam = interaction.options.getString('name', true);
	const editContent = interaction.options.getString('content', true);

	try {
		await editMacro(editNameParam, editContent, guildId);
		await interaction.reply({
			content: `✅ Macro "${editNameParam}" edited successfully.`,
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	} catch (error) {
		await interaction.reply({
			content: `❌ Failed to edit macro.`,
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	}
}
