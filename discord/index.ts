import { Client, GatewayIntentBits, Events } from "discord.js";
import "dotenv/config";
import { handleSlashCommand } from "./commands/index.js";
import { handleDirectMessage } from "./handlers/dmHandler.js";
import { handleChannelMessage } from "./handlers/channelHandler.js";

// Environment variables
const DISCORD_BOT_TOKEN = process.env.DISCORD_BOT_TOKEN!;

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

// Handle slash commands
client.on(Events.InteractionCreate, async (interaction) => {
  if (!interaction.isChatInputCommand()) return;
  await handleSlashCommand(interaction, client);
});

// Handle direct messages
client.on(Events.MessageCreate, async (message) => {
  await handleDirectMessage(message, client);
});

// Handle messages in modmail channels (relay to user)
client.on(Events.MessageCreate, async (message) => {
  await handleChannelMessage(message, client);
});

// Login to Discord
client.login(DISCORD_BOT_TOKEN);
