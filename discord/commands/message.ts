import { ChatInputCommandInteraction, ChannelType, Client } from "discord.js";
import { getThreadByUserId, createThread, addMessageToThread } from "../api.js";
import { 
  createModeratorMessageEmbed, 
  generateWelcomeEmbed, 
  createConfirmationEmbed, 
  generateChannelName 
} from "../utils.js";

const MODMAIL_CATEGORY_ID = process.env.PUBLIC_DISCORD_MODMAIL_CHANNEL_ID!;

export async function handleMessageCommand(
  interaction: ChatInputCommandInteraction,
  client: Client
) {
  const user = interaction.options.getUser("user", true);
  const messageContent = interaction.options.getString("message", true);

  try {
    // Send DM to user
    const embed = createModeratorMessageEmbed(messageContent);
    await user.send({ embeds: [embed] });

    // Find existing thread or create a new one
    let thread = await getThreadByUserId(user.id);
    let channel;

    if (!thread) {
      // Create new channel for this user
      const guild = interaction.guild!;
      channel = await guild.channels.create({
        name: generateChannelName(user),
        type: ChannelType.GuildText,
        parent: MODMAIL_CATEGORY_ID,
        topic: `Modmail thread for ${user.tag} (${user.id})`,
      });

      const welcomeEmbed = generateWelcomeEmbed(user, guild);

      if (channel.isTextBased() && "send" in channel) {
        await channel.send({ embeds: [welcomeEmbed] });
      }

      // Create thread in database
      thread = await createThread(user.id, channel.id);
    } else {
      channel = await client.channels.fetch(thread.thread_id);
    }

    // Add message to thread
    await addMessageToThread(
      thread.id,
      interaction.user.id,
      interaction.user.tag,
      messageContent
    );

    // Send confirmation to channel
    const confirmEmbed = createConfirmationEmbed(user, messageContent);

    if (channel?.isTextBased() && "send" in channel) {
      await channel.send({ embeds: [confirmEmbed] });
    }

    await interaction.reply({
      content: `✅ Message sent to ${user.tag}`,
      ephemeral: true,
    });
  } catch (error) {
    console.error("Error sending message:", error);
    await interaction.reply({
      content: "❌ Failed to send message. Please check the user ID.",
      ephemeral: true,
    });
  }
} 