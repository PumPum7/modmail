import { Message, Client } from 'discord.js';
import { getThreadByChannelId, addMessageToThread } from '../api.js';
import {
	createModeratorMessageEmbed,
	processAttachments,
	addAttachmentsToEmbed,
} from '../utils.js';

export async function handleChannelMessage(message: Message, client: Client) {
	// Ignore bot messages
	if (message.author.bot) return;

	// Only handle messages in guild channels
	if (!message.guild) return;

	const guildId = message.guild.id;

	// Check if this is a modmail thread
	const thread = await getThreadByChannelId(message.channelId, guildId);

	if (!thread || !thread.is_open) return;

	// Don't relay slash command interactions
	if (message.content.startsWith('/')) return;

	try {
		// Get the user
		const user = await client.users.fetch(thread.user_id);

		// Process attachments
		const attachments = processAttachments(Array.from(message.attachments.values()));

		// Send message to user
		const embed = createModeratorMessageEmbed(message.content || '*No text content*');
		addAttachmentsToEmbed(embed, attachments);

		await user.send({ embeds: [embed] });

		// Add to thread
		await addMessageToThread(
			thread.id,
			guildId,
			message.author.id,
			message.author.tag,
			message.content,
			attachments
		);

		// React to confirm message was sent
		await message.react('✅');
	} catch (error) {
		console.error('Error relaying message to user:', error);
		await message.react('❌');
	}
}
