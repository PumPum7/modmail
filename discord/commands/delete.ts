import { ChatInputCommandInteraction, Client } from "discord.js";

export async function handleDeleteCommand(
  interaction: ChatInputCommandInteraction,
  client: Client
) {
  // Delete the discord channel
  const channel = await client.channels.fetch(interaction.channelId);
  if (channel) {
    await channel.delete();
  }
} 