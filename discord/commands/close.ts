import { ChatInputCommandInteraction, Client, MessageFlagsBitField } from 'discord.js';
import { getThreadByChannelId, closeThread } from '../api.js';
import {
	createThreadClosedEmbed,
	createLogEmbed,
	createUserClosureNotificationEmbed,
} from '../utils.js';

const LOG_CHANNEL_ID = process.env.PUBLIC_LOG_CHANNEL;
const FRONTEND_URL = process.env.PUBLIC_FRONT_END_URL;

export async function handleCloseCommand(interaction: ChatInputCommandInteraction, client: Client) {
	const channelId = interaction.channelId;
	const thread = await getThreadByChannelId(channelId);

	if (!thread) {
		await interaction.reply({
			content: '❌ This is not a modmail thread.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
		return;
	}

	if (!thread.is_open) {
		await interaction.reply({
			content: '❌ This thread is already closed.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
		return;
	}

	// Close thread in database
	await closeThread(thread.id);

	// Send closure message
	const embed = createThreadClosedEmbed(interaction.user);
	await interaction.reply({ embeds: [embed] });

	// Post to log channel if configured
	if (LOG_CHANNEL_ID && FRONTEND_URL) {
		try {
			const logChannel = await client.channels.fetch(LOG_CHANNEL_ID);
			if (logChannel?.isTextBased() && 'send' in logChannel) {
				const user = await client.users.fetch(thread.user_id);
				const logEmbed = createLogEmbed(user, interaction.user, thread.id);
				await logChannel.send({ embeds: [logEmbed] });
			}
		} catch (error) {
			console.error('Error posting to log channel:', error);
		}
	}

	// Notify user
	try {
		const user = await client.users.fetch(thread.user_id);
		const userEmbed = createUserClosureNotificationEmbed();
		await user.send({ embeds: [userEmbed] });
	} catch (error) {
		console.error('Error notifying user of closure:', error);
	}
}
