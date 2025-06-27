import { Client, GatewayIntentBits, Events } from "discord.js";
import express, { type Request, type Response } from "express";
import "dotenv/config";
import { handleSlashCommand } from "./commands/index.js";
import { handleDirectMessage } from "./handlers/dmHandler.js";
import { handleChannelMessage } from "./handlers/channelHandler.js";
import { handleWebhookThreadClosed } from "./webhookHandler.js";
import { handleButtonInteraction } from "./handlers/buttonHandler.js";

// Environment variables
const DISCORD_BOT_TOKEN = process.env.DISCORD_BOT_TOKEN!;
const WEBHOOK_PORT = process.env.WEBHOOK_PORT || 3001;

// Create Discord client
const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.DirectMessages,
    GatewayIntentBits.MessageContent,
  ],
});

client.once(Events.ClientReady, (readyClient) => {
  console.log(`Ready! Logged in as ${readyClient.user.tag}`);
});

// Handle interactions (slash commands and buttons)
client.on(Events.InteractionCreate, async (interaction) => {
  if (interaction.isChatInputCommand()) {
    await handleSlashCommand(interaction, client);
  } else if (interaction.isButton()) {
    await handleButtonInteraction(interaction, client);
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
