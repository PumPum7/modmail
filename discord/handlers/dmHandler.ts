import {
	Message,
	Client,
	ChannelType,
	ActionRowBuilder,
	ButtonBuilder,
	ButtonStyle,
} from 'discord.js';
import {
	getThreadByUserId,
	createThread,
	addMessageToThread,
	isUserBlocked,
	getMacros,
} from '../api.js';
import {
	generateWelcomeEmbed,
	createUserMessageEmbed,
	processAttachments,
	addAttachmentsToEmbed,
	generateChannelName,
	createUserConfirmationEmbed,
	createQuickReplyButtons,
	createIntroModal,
	createIntroPromptEmbed,
} from '../utils.js';

const MODMAIL_CATEGORY_ID = process.env.PUBLIC_DISCORD_MODMAIL_CHANNEL_ID!;

export async function handleDirectMessage(message: Message, client: Client) {
	// Ignore bot messages and messages from guilds
	if (message.author.bot || message.channel.type !== ChannelType.DM) return;

	const userId = message.author.id;

	try {
		// Check if user is blocked
		const isBlocked = await isUserBlocked(userId);
		if (isBlocked) {
			console.log(`Blocked user ${message.author.tag} (${userId}) tried to send a DM`);
			return; // Silently ignore messages from blocked users
		}

		// Check if user already has an open thread
		let thread = await getThreadByUserId(userId);

		if (!thread) {
			// This is a first-time user - show intro modal instead of creating thread immediately
			const introEmbed = createIntroPromptEmbed();
			const modal = createIntroModal(userId);

			// Create a button to trigger the intro modal
			const introButton = new ButtonBuilder()
				.setCustomId(`start_intro_${userId}`)
				.setLabel('Start Conversation')
				.setStyle(ButtonStyle.Primary);

			const cancelButton = new ButtonBuilder()
				.setCustomId(`cancel_intro_${userId}`)
				.setLabel('Cancel')
				.setStyle(ButtonStyle.Danger);

			const row = new ActionRowBuilder<ButtonBuilder>().addComponents(introButton, cancelButton);

			try {
				await message.author.send({
					embeds: [introEmbed],
					components: [row],
				});
				console.log(`Sent intro prompt to new user ${message.author.tag} (${userId})`);
			} catch (error) {
				console.error('Failed to send intro prompt:', error);
				// Fallback to immediate thread creation if DM fails
				await createThreadForUser(message, client);
			}
			return;
		}

		// Existing thread - continue with normal flow
		await handleExistingThread(message, client, thread);
	} catch (error) {
		console.error('Error handling DM:', error);

		try {
			await message.author.send(
				'❌ There was an error processing your message. Please try again later.'
			);
		} catch (dmError) {
			console.error('Could not send error message to user:', dmError);
		}
	}
}

async function createThreadForUser(message: Message, client: Client, introData?: any) {
	const guild = client.guilds.cache.first();
	if (!guild) {
		console.error('No guild found');
		return null;
	}

	const channel = await guild.channels.create({
		name: generateChannelName(message.author),
		type: ChannelType.GuildText,
		parent: MODMAIL_CATEGORY_ID,
		topic: `Modmail thread for ${message.author.tag} (${message.author.id})`,
	});

	// Create thread in database with urgency from intro data
	const urgency = introData?.urgency || 'Medium';
	const thread = await createThread(message.author.id, channel.id, urgency);

	// Send welcome message to channel with intro data if available
	const welcomeEmbed = generateWelcomeEmbed(message.author, guild, introData);

	if (channel.isTextBased() && 'send' in channel) {
		await channel.send({ embeds: [welcomeEmbed] });
	}

	return { thread, channel };
}

async function handleExistingThread(message: Message, client: Client, thread: any) {
	const channel = await client.channels.fetch(thread.thread_id);

	// Process attachments
	const attachments = processAttachments(Array.from(message.attachments.values()));

	// Add message to thread
	await addMessageToThread(
		thread.id,
		message.author.id,
		message.author.tag,
		message.content,
		attachments
	);

	// Forward message to modmail channel
	const messageEmbed = createUserMessageEmbed(message.author, message.content);
	addAttachmentsToEmbed(messageEmbed, attachments);

	const macros = await getMacros();
	const components = createQuickReplyButtons(macros);

	if (channel?.isTextBased() && 'send' in channel) {
		await channel.send({
			embeds: [messageEmbed],
			components: components,
		});
	}

	// Send confirmation to user
	const confirmEmbed = createUserConfirmationEmbed();
	await message.author.send({ embeds: [confirmEmbed] });
}

// Export function for use in interaction handler
export { createThreadForUser };
