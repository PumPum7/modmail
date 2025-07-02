import { ChatInputCommandInteraction, ChannelType, Client, MessageFlagsBitField } from 'discord.js';
import { getThreadByUserId, createThread, addMessageToThread, getGuildConfig } from '../api.js';
import {
	createModeratorMessageEmbed,
	generateWelcomeEmbed,
	createConfirmationEmbed,
	generateChannelName,
} from '../utils.js';

export async function handleMessageCommand(
	interaction: ChatInputCommandInteraction,
	client: Client
) {
	const user = interaction.options.getUser('user', true);
	const messageContent = interaction.options.getString('message', true);
	const guildId = interaction.guildId!;

	try {
		// Get guild configuration
		let guildConfig;
		try {
			guildConfig = await getGuildConfig(guildId);
		} catch (error) {
			await interaction.reply({
				content: '❌ Guild configuration not found. Please run `/config show` to set up the bot.',
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
			return;
		}

		const MODMAIL_CATEGORY_ID = guildConfig.modmail_category_id;
		if (!MODMAIL_CATEGORY_ID) {
			await interaction.reply({
				content:
					'❌ Modmail category not configured. Please use `/config set modmail-category` to set it up.',
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
			return;
		}

		// Send DM to user
		const embed = createModeratorMessageEmbed(messageContent);
		await user.send({ embeds: [embed] });

		// Find existing thread or create a new one
		let thread = await getThreadByUserId(user.id, guildId);
		let channel;

		if (!thread) {
			// Create new channel for this user
			const guild = interaction.guild!;
			channel = await guild.channels.create({
				name: generateChannelName(user, guildConfig.randomize_names),
				type: ChannelType.GuildText,
				parent: MODMAIL_CATEGORY_ID,
				topic: `Modmail thread for ${user.tag} (${user.id})`,
			});

			const welcomeEmbed = generateWelcomeEmbed(user, guild, guildConfig.welcome_message);

			if (channel.isTextBased() && 'send' in channel) {
				await channel.send({ embeds: [welcomeEmbed] });
			}

			// Create thread in database
			thread = await createThread(user.id, channel.id, guildId, 'Medium');
		} else {
			channel = await client.channels.fetch(thread.thread_id);
		}

		// Add message to thread
		await addMessageToThread(
			thread.id,
			guildId,
			interaction.user.id,
			interaction.user.tag,
			messageContent
		);

		// Send confirmation to channel
		const confirmEmbed = createConfirmationEmbed(user, messageContent);

		if (channel?.isTextBased() && 'send' in channel) {
			await channel.send({ embeds: [confirmEmbed] });
		}

		await interaction.reply({
			content: `✅ Message sent to ${user.tag}`,
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	} catch (error) {
		console.error('Error sending message:', error);
		await interaction.reply({
			content: '❌ Failed to send message. Please check the user ID.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	}
}
