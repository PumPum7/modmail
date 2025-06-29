import { ChatInputCommandInteraction, MessageFlagsBitField } from 'discord.js';
import { getThreadByChannelId, updateThreadUrgency } from '../api.js';

export async function handleUrgencyCommand(interaction: ChatInputCommandInteraction) {
	const urgency = interaction.options.getString('level', true);

	try {
		// Get thread from current channel
		const thread = await getThreadByChannelId(interaction.channelId);

		if (!thread) {
			await interaction.reply({
				content: '❌ This command can only be used in modmail threads.',
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
			return;
		}

		if (!thread.is_open) {
			await interaction.reply({
				content: '❌ Cannot change urgency of a closed thread.',
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
			return;
		}

		// Validate urgency level
		const validLevels = ['Low', 'Medium', 'High', 'Urgent'];
		const normalizedUrgency = urgency.charAt(0).toUpperCase() + urgency.slice(1).toLowerCase();

		if (!validLevels.includes(normalizedUrgency)) {
			await interaction.reply({
				content: '❌ Invalid urgency level. Valid levels are: Low, Medium, High, Urgent',
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
			return;
		}

		// Update thread urgency
		await updateThreadUrgency(thread.id, normalizedUrgency);

		await interaction.reply({
			content: `✅ Thread urgency updated to **${normalizedUrgency}**`,
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});

		// Send notification to channel
		if (interaction.channel?.isTextBased() && 'send' in interaction.channel) {
			await interaction.channel.send(
				`🔄 **Thread urgency changed to ${normalizedUrgency}** by ${interaction.user.tag}`
			);
		}
	} catch (error) {
		console.error('Error updating thread urgency:', error);
		await interaction.reply({
			content: '❌ Failed to update thread urgency. Please try again.',
			flags: MessageFlagsBitField.Flags.Ephemeral,
		});
	}
}
