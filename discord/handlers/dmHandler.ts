import {
	Message,
	Client,
	ChannelType,
	ActionRowBuilder,
	ButtonBuilder,
	ButtonStyle,
	StringSelectMenuBuilder,
	StringSelectMenuOptionBuilder,
} from 'discord.js';
import {
	getThreadByUserId,
	createThread,
	addMessageToThread,
	isUserBlocked,
	getMacros,
	getUserServers,
	getGuildConfig,
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

export async function handleDirectMessage(message: Message, client: Client) {
	// Ignore bot messages and messages from guilds
	if (message.author.bot || message.channel.type !== ChannelType.DM) return;

	const userId = message.author.id;

	try {
		// Get user's available servers
		const userServers = await getUserServers(userId);

		if (userServers.length === 0) {
			await message.author.send(
				'❌ You are not a member of any servers that have this modmail bot configured. Please join a server first.'
			);
			return;
		}

		// If user is only in one server, proceed directly
		if (userServers.length === 1) {
			const guildId = userServers[0].guild_id;
			await handleSingleServerFlow(message, client, guildId);
			return;
		}

		// Multiple servers - check if user has existing threads
		const existingThreads = await Promise.all(
			userServers.map(async (server) => {
				const thread = await getThreadByUserId(userId, server.guild_id);
				return thread ? { ...thread, server } : null;
			})
		);

		const activeThreads = existingThreads.filter((t) => t && t.is_open);

		if (activeThreads.length === 1) {
			// User has one active thread, continue with that server
			await handleExistingThread(message, client, activeThreads[0]);
			return;
		}

		if (activeThreads.length > 1) {
			// Multiple active threads - let user choose
			await showServerSelection(message, activeThreads, 'existing');
			return;
		}

		// No active threads - show server selection for new thread
		await showServerSelection(message, userServers, 'new');
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

async function handleSingleServerFlow(message: Message, client: Client, guildId: string) {
	const userId = message.author.id;

	try {
		// Check if user is blocked in this server
		const isBlocked = await isUserBlocked(userId, guildId);
		if (isBlocked) {
			console.log(`Blocked user ${message.author.tag} (${userId}) tried to send a DM`);
			return; // Silently ignore messages from blocked users
		}

		// Check if user already has an open thread in this server
		let thread = await getThreadByUserId(userId, guildId);

		if (!thread) {
			// This is a first-time user - show intro modal instead of creating thread immediately
			const introEmbed = createIntroPromptEmbed();

			// Create a button to trigger the intro modal
			const introButton = new ButtonBuilder()
				.setCustomId(`start_intro_${userId}_${guildId}`)
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
				await createThreadForUser(message, client, guildId);
			}
			return;
		}

		// Existing thread - continue with normal flow
		await handleExistingThread(message, client, thread);
	} catch (error) {
		console.error('Error in single server flow:', error);
		throw error;
	}
}

async function showServerSelection(message: Message, servers: any[], type: 'new' | 'existing') {
	const options = servers.map((server, index) => {
		const serverData = type === 'existing' ? server.server : server;
		return new StringSelectMenuOptionBuilder()
			.setLabel(serverData.guild_name || `Server ${index + 1}`)
			.setValue(serverData.guild_id)
			.setDescription(
				type === 'existing' ? 'Continue existing conversation' : 'Start new conversation'
			);
	});

	const selectMenu = new StringSelectMenuBuilder()
		.setCustomId(`server_select_${type}_${message.author.id}`)
		.setPlaceholder('Choose a server to contact...')
		.addOptions(options);

	const row = new ActionRowBuilder<StringSelectMenuBuilder>().addComponents(selectMenu);

	const title = type === 'existing' ? 'Continue Existing Conversation' : 'Choose Server to Contact';

	const description =
		type === 'existing'
			? 'You have active conversations in multiple servers. Which one would you like to continue?'
			: 'You are a member of multiple servers with this modmail bot. Which server would you like to contact?';

	await message.author.send({
		content: `**${title}**\n${description}`,
		components: [row],
	});
}

async function createThreadForUser(
	message: Message,
	client: Client,
	guildId: string,
	introData?: any
) {
	try {
		const guildConfig = await getGuildConfig(guildId);
		const MODMAIL_CATEGORY_ID = guildConfig.modmail_category_id;

		if (!MODMAIL_CATEGORY_ID) {
			await message.author.send(
				'❌ This server has not configured modmail properly. Please contact an administrator.'
			);
			return null;
		}

		const guild = client.guilds.cache.get(guildId);
		if (!guild) {
			console.error(`Guild ${guildId} not found`);
			return null;
		}

		const channel = await guild.channels.create({
			name: generateChannelName(message.author, guildConfig.randomize_names),
			type: ChannelType.GuildText,
			parent: MODMAIL_CATEGORY_ID,
			topic: `Modmail thread for ${message.author.tag} (${message.author.id})`,
		});

		// Create thread in database with urgency from intro data
		const urgency = introData?.urgency || 'Medium';
		const thread = await createThread(message.author.id, channel.id, guildId, urgency);

		// Send welcome message to channel with intro data if available
		const welcomeEmbed = generateWelcomeEmbed(
			message.author,
			guild,
			guildConfig.welcome_message,
			introData
		);

		if (channel.isTextBased() && 'send' in channel) {
			await channel.send({ embeds: [welcomeEmbed] });
		}

		return { thread, channel };
	} catch (error) {
		console.error('Error creating thread for user:', error);
		await message.author.send('❌ Failed to create modmail thread. Please try again later.');
		return null;
	}
}

async function handleExistingThread(message: Message, client: Client, thread: any) {
	const channel = await client.channels.fetch(thread.thread_id);

	// Process attachments
	const attachments = processAttachments(Array.from(message.attachments.values()));

	// Add message to thread
	await addMessageToThread(
		thread.id,
		thread.guild_id,
		message.author.id,
		message.author.tag,
		message.content,
		attachments
	);

	// Forward message to modmail channel
	const messageEmbed = createUserMessageEmbed(message.author, message.content);
	addAttachmentsToEmbed(messageEmbed, attachments);

	const macros = await getMacros(thread.guild_id);
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
