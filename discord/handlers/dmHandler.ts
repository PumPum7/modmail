import { Message, Client, ChannelType } from "discord.js";
import { getThreadByUserId, createThread, addMessageToThread, isUserBlocked, getMacros } from "../api.js";
import { 
  generateWelcomeEmbed, 
  createUserMessageEmbed, 
  processAttachments, 
  addAttachmentsToEmbed,
  generateChannelName,
  createUserConfirmationEmbed,
  createQuickReplyButtons
} from "../utils.js";

const MODMAIL_CATEGORY_ID = process.env.PUBLIC_DISCORD_MODMAIL_CHANNEL_ID!;

export async function handleDirectMessage(message: Message, client: Client) {
  // Ignore bot messages and messages from guilds
  if (message.author.bot || message.guild) return;

  const userId = message.author.id;

  try {
    // Check if user is blocked
    const isBlocked = await isUserBlocked(userId);
    if (isBlocked) {
      console.log(
        `Blocked user ${message.author.tag} (${userId}) tried to send a DM`
      );
      return; // Silently ignore messages from blocked users
    }

    // Check if user already has an open thread
    let thread = await getThreadByUserId(userId);
    let channel;

    if (!thread) {
      // Create new channel
      const guild = client.guilds.cache.first();
      if (!guild) {
        console.error("No guild found");
        return;
      }

      channel = await guild.channels.create({
        name: generateChannelName(message.author),
        type: ChannelType.GuildText,
        parent: MODMAIL_CATEGORY_ID,
        topic: `Modmail thread for ${message.author.tag} (${message.author.id})`,
      });

      // Create thread in database
      thread = await createThread(userId, channel.id);

      // Send welcome message to channel
      const welcomeEmbed = generateWelcomeEmbed(message.author, guild);

      if (channel.isTextBased() && "send" in channel) {
        await channel.send({ embeds: [welcomeEmbed] });
      }
    } else {
      channel = await client.channels.fetch(thread.thread_id);
    }

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

    if (channel?.isTextBased() && "send" in channel) {
      await channel.send({ 
        embeds: [messageEmbed],
        components: components
      });
    }

    // Send confirmation to user
    const confirmEmbed = createUserConfirmationEmbed();
    await message.author.send({ embeds: [confirmEmbed] });
    
  } catch (error) {
    console.error("Error handling DM:", error);

    try {
      await message.author.send(
        "❌ There was an error processing your message. Please try again later."
      );
    } catch (dmError) {
      console.error("Could not send error message to user:", dmError);
    }
  }
}