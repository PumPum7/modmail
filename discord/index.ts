import { Client, GatewayIntentBits, Events, MessageFlagsBitField, Partials } from 'discord.js';
import express, { type Request, type Response } from 'express';
import 'dotenv/config';
import { handleSlashCommand } from './commands/index.js';
import { handleDirectMessage } from './handlers/dmHandler.js';
import { handleChannelMessage } from './handlers/channelHandler.js';
import { handleWebhookThreadClosed } from './webhookHandler.js';
import { handleButtonInteraction } from './handlers/buttonHandler.js';

// Environment variables
const DISCORD_BOT_TOKEN = process.env.DISCORD_BOT_TOKEN!;
const WEBHOOK_PORT = process.env.WEBHOOK_PORT || 3001;

// Create Discord client
const client = new Client({
	intents: [
		GatewayIntentBits.Guilds,
		GatewayIntentBits.GuildMessages,
		GatewayIntentBits.GuildMembers,
		GatewayIntentBits.DirectMessages,
		GatewayIntentBits.MessageContent,
	],
	partials: [Partials.Channel],
});

client.once(Events.ClientReady, (readyClient) => {
	console.log(`Ready! Logged in as ${readyClient.user.tag}`);
});

// Handle interactions (slash commands, buttons, and modals)
client.on(Events.InteractionCreate, async (interaction) => {
	if (interaction.isChatInputCommand()) {
		await handleSlashCommand(interaction, client);
	} else if (interaction.isButton()) {
		// Handle intro button clicks
		if (interaction.customId.startsWith('start_intro_')) {
			const userId = interaction.customId.replace('start_intro_', '');

			// Verify this is the correct user
			if (interaction.user.id !== userId) {
				await interaction.reply({
					content: '❌ This button is not for you.',
					flags: MessageFlagsBitField.Flags.Ephemeral,
				});
				return;
			}

			const { createIntroModal } = await import('./utils.js');
			const modal = createIntroModal(userId);

			try {
				await interaction.showModal(modal);
			} catch (error) {
				console.error('Error showing intro modal:', error);
				await interaction.reply({
					content: '❌ There was an error showing the form. Please try again.',
					flags: MessageFlagsBitField.Flags.Ephemeral,
				});
			}
		} else if (interaction.customId.startsWith('cancel_intro_')) {
			await interaction.reply({
				content: '❌ Intro cancelled.',
				flags: MessageFlagsBitField.Flags.Ephemeral,
			});
		} else {
			// Handle other button interactions (quick replies, etc.)
			await handleButtonInteraction(interaction, client);
		}
	} else if (interaction.isModalSubmit()) {
		// Handle intro modal submissions
		if (interaction.customId.startsWith('intro_modal_')) {
			const userId = interaction.customId.replace('intro_modal_', '');

			// Verify this is the correct user
			if (interaction.user.id !== userId) {
				await interaction.reply({
					content: '❌ This form is not for you.',
					flags: MessageFlagsBitField.Flags.Ephemeral,
				});
				return;
			}

			try {
				// Extract form data
				const subject = interaction.fields.getTextInputValue('subject');
				const description = interaction.fields.getTextInputValue('description');
				const urgency = interaction.fields.getTextInputValue('urgency') || 'Not specified';

				const introData = {
					subject,
					description,
					urgency,
				};

				// Create the thread with intro data
				const { createThreadForUser } = await import('./handlers/dmHandler.js');

				// Create a mock message object for thread creation
				const mockMessage = {
					author: interaction.user,
					content: `[INTRO FORM] ${subject}: ${description}`,
					attachments: new Map(),
				};

				const result = await createThreadForUser(mockMessage as any, client, introData);

				if (result) {
					const { thread } = result;

					// Add the initial intro message to the thread
					const { addMessageToThread } = await import('./api.js');
					await addMessageToThread(
						thread.id,
						interaction.user.id,
						interaction.user.tag,
						`[INTRO FORM]\n**Subject:** ${subject}\n**Description:** ${description}\n**Priority:** ${urgency}`
					);

					await interaction.reply({
						content:
							'✅ Thank you! Your modmail thread has been created. A moderator will respond to you shortly.',
						flags: MessageFlagsBitField.Flags.Ephemeral,
					});

					console.log(
						`Created thread ${thread.id} for user ${interaction.user.tag} (${interaction.user.id}) via intro form`
					);
				} else {
					throw new Error('Failed to create thread');
				}
			} catch (error) {
				console.error('Error processing intro modal:', error);
				await interaction.reply({
					content:
						'❌ There was an error processing your information. Please try contacting us again.',
					flags: MessageFlagsBitField.Flags.Ephemeral,
				});
			}
		}
	}
});

// Handle direct messages
client.on(Events.MessageCreate, async (message) => {
	await handleDirectMessage(message, client);
});

// Handle messages in modmail channels (relay to user)
client.on(Events.MessageCreate, async (message) => {
	await handleChannelMessage(message, client);
});

// Create webhook server
const app = express();
app.use(express.json());

app.post('/webhook', async (req: Request, res: Response) => {
	try {
		const { type, ...payload } = req.body;

		switch (type) {
			case 'thread_closed':
				await handleWebhookThreadClosed(payload, client);
				break;
			default:
				console.log('Unknown webhook type:', type);
		}

		res.status(200).json({ success: true });
	} catch (error) {
		console.error('Webhook error:', error);
		res.status(500).json({ error: 'Internal server error' });
	}
});

app.listen(WEBHOOK_PORT, () => {
	console.log(`Webhook server listening on port ${WEBHOOK_PORT}`);
});

// Login to Discord
client.login(DISCORD_BOT_TOKEN);
// test comment
