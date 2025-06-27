import {
  Client,
  GatewayIntentBits,
  Events,
  ChatInputCommandInteraction,
  ChannelType,
  EmbedBuilder,
} from "discord.js";
import "dotenv/config";

// Types for API responses
interface Thread {
  id: number;
  user_id: string;
  thread_id: string;
  is_open: boolean;
}

interface MessageData {
  id: string;
  author_id: string;
  author_tag: string;
  content: string;
  created_at: string;
}

interface Macro {
  id: number;
  name: string;
  content: string;
}

// Environment variables
const DISCORD_BOT_TOKEN = process.env.DISCORD_BOT_TOKEN!;
const MODMAIL_CATEGORY_ID = process.env.PUBLIC_DISCORD_MODMAIL_CHANNEL_ID!;
const BACKEND_URL = process.env.PUBLIC_BACKEND_URL || "http://localhost:8080";

// Create Discord client
const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.DirectMessages,
    GatewayIntentBits.MessageContent,
  ],
});

// API helper functions
async function createThread(
  userId: string,
  channelId: string
): Promise<Thread> {
  const response = await fetch(`${BACKEND_URL}/threads`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      user_id: userId,
      thread_id: channelId,
    }),
  });
  return response.json() as Promise<Thread>;
}

async function getThreadByUserId(userId: string): Promise<Thread | null> {
  const response = await fetch(`${BACKEND_URL}/threads`);
  const threads: Thread[] = (await response.json()) as Thread[];
  return threads.find((t) => t.user_id === userId && t.is_open) || null;
}

async function closeThread(threadId: number): Promise<Thread> {
  const response = await fetch(`${BACKEND_URL}/threads/${threadId}/close`, {
    method: "POST",
  });
  return response.json() as Promise<Thread>;
}

async function addMessageToThread(
  threadId: number,
  authorId: string,
  authorTag: string,
  content: string
): Promise<MessageData> {
  const response = await fetch(`${BACKEND_URL}/threads/${threadId}/messages`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      author_id: authorId,
      author_tag: authorTag,
      content: content,
    }),
  });
  return response.json() as Promise<MessageData>;
}

async function addNoteToThread(
  threadId: number,
  authorId: string,
  authorTag: string,
  content: string
): Promise<any> {
  const response = await fetch(`${BACKEND_URL}/threads/${threadId}/notes`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      author_id: authorId,
      author_tag: authorTag,
      content: content,
    }),
  });
  return response.json();
}

async function createMacro(name: string, content: string): Promise<Macro> {
  const response = await fetch(`${BACKEND_URL}/macros`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ name, content }),
  });
  return response.json() as Promise<Macro>;
}

async function getMacroByName(name: string): Promise<Macro | null> {
  const response = await fetch(
    `${BACKEND_URL}/macros/${encodeURIComponent(name)}`
  );
  const result = await response.json();
  return result === null ? null : (result as Macro);
}

async function deleteMacro(
  name: string
): Promise<{ success: boolean; message: string }> {
  const response = await fetch(
    `${BACKEND_URL}/macros/${encodeURIComponent(name)}`,
    {
      method: "DELETE",
    }
  );
  return response.json() as Promise<{ success: boolean; message: string }>;
}

async function getMacros(): Promise<Macro[]> {
  const response = await fetch(`${BACKEND_URL}/macros`);
  return response.json() as Promise<Macro[]>;
}

async function editMacro(name: string, content: string): Promise<Macro> {
  const response = await fetch(
    `${BACKEND_URL}/macros/${encodeURIComponent(name)}`,
    {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ content }),
    }
  );
  return response.json() as Promise<Macro>;
}

client.once(Events.ClientReady, (readyClient) => {
  console.log(`Ready! Logged in as ${readyClient.user.tag}`);
});

// Handle slash commands
client.on(Events.InteractionCreate, async (interaction) => {
  if (!interaction.isChatInputCommand()) return;

  const { commandName } = interaction;

  try {
    switch (commandName) {
      case "message":
        await handleMessageCommand(interaction);
        break;
      case "close":
        await handleCloseCommand(interaction);
        break;
      case "note":
        await handleNoteCommand(interaction);
        break;
      case "macro":
        await handleMacroCommand(interaction);
        break;
      case "delete":
        await handleDeleteCommand(interaction);
        break;
    }
  } catch (error) {
    console.error("Error handling command:", error);
    const reply = {
      content: "❌ An error occurred while processing your command.",
      ephemeral: true,
    };

    if (interaction.replied || interaction.deferred) {
      await interaction.followUp(reply);
    } else {
      await interaction.reply(reply);
    }
  }
});

async function handleMessageCommand(interaction: ChatInputCommandInteraction) {
  const user = interaction.options.getUser("user", true);
  const messageContent = interaction.options.getString("message", true);

  try {
    // Try to get the user
    // Send DM to user
    const embed = new EmbedBuilder()
      .setColor(0x0099ff)
      .setTitle("Message from Moderators")
      .setDescription(messageContent)
      .setTimestamp();

    await user.send({ embeds: [embed] });

    // Find existing thread or create a new one
    let thread = await getThreadByUserId(user.id);
    let channel;

    if (!thread) {
      // Create new channel for this user
      const guild = interaction.guild!;
      channel = await guild.channels.create({
        name: `${user.username}-${user.discriminator}`,
        type: ChannelType.GuildText,
        parent: MODMAIL_CATEGORY_ID,
        topic: `Modmail thread for ${user.tag} (${user.id})`,
      });

      const welcomeEmbed = new EmbedBuilder()
        .setColor(0x0099ff)
        .setTitle("New Modmail Thread")
        .setDescription(
          `**User:** ${user.tag} (${
            user.id
          })\n**Account Created:** ${user.createdAt.toLocaleDateString()}\n**User Joined:** ${guild.members.cache
            .get(user.id)
            ?.joinedAt?.toLocaleDateString()}`
        )
        .setThumbnail(user.displayAvatarURL())
        .setTimestamp();

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
    const confirmEmbed = new EmbedBuilder()
      .setColor(0x00ff00)
      .setAuthor({
        name: interaction.user.tag,
        iconURL: interaction.user.displayAvatarURL(),
      })
      .setDescription(`**Message sent to ${user.tag}:**\n${messageContent}`)
      .setTimestamp();

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

async function handleCloseCommand(interaction: ChatInputCommandInteraction) {
  const channelId = interaction.channelId;

  // Find thread by channel ID
  const response = await fetch(`${BACKEND_URL}/threads`);
  const threads: Thread[] = (await response.json()) as Thread[];
  const thread = threads.find((t) => t.thread_id === channelId);

  if (!thread) {
    await interaction.reply({
      content: "❌ This is not a modmail thread.",
      ephemeral: true,
    });
    return;
  }

  if (!thread.is_open) {
    await interaction.reply({
      content: "❌ This thread is already closed.",
      ephemeral: true,
    });
    return;
  }

  // Close thread in database
  await closeThread(thread.id);

  // Send closure message
  const embed = new EmbedBuilder()
    .setColor(0xff0000)
    .setTitle("Thread Closed")
    .setDescription(`This thread has been closed by ${interaction.user.tag}`)
    .setTimestamp();

  await interaction.reply({ embeds: [embed] });

  // Notify user
  try {
    const user = await client.users.fetch(thread.user_id);
    const userEmbed = new EmbedBuilder()
      .setColor(0xff0000)
      .setTitle("Modmail Thread Closed")
      .setDescription("Your modmail thread has been closed by the moderators.")
      .setTimestamp();

    await user.send({ embeds: [userEmbed] });
  } catch (error) {
    console.error("Error notifying user of closure:", error);
  }
}

async function handleNoteCommand(interaction: ChatInputCommandInteraction) {
  const noteContent = interaction.options.getString("content", true);
  const channelId = interaction.channelId;

  // Find thread by channel ID
  const response = await fetch(`${BACKEND_URL}/threads`);
  const threads: Thread[] = (await response.json()) as Thread[];
  const thread = threads.find((t) => t.thread_id === channelId);

  if (!thread) {
    await interaction.reply({
      content: "❌ This command can only be used in a modmail thread.",
      ephemeral: true,
    });
    return;
  }

  try {
    // Add note to thread
    await addNoteToThread(
      thread.id,
      interaction.user.id,
      interaction.user.tag,
      noteContent
    );

    await interaction.reply({
      content: `✅ Internal note added to thread.`,
      ephemeral: true,
    });
  } catch (error) {
    console.error("Error adding note:", error);
    await interaction.reply({
      content: "❌ Failed to add note.",
      ephemeral: true,
    });
  }
}

async function handleMacroCommand(interaction: ChatInputCommandInteraction) {
  const subcommand = interaction.options.getSubcommand();

  switch (subcommand) {
    case "create":
      const name = interaction.options.getString("name", true);
      const content = interaction.options.getString("content", true);

      try {
        await createMacro(name, content);
        await interaction.reply({
          content: `✅ Macro "${name}" created successfully.`,
          ephemeral: true,
        });
      } catch (error) {
        await interaction.reply({
          content: `❌ Failed to create macro. It may already exist.`,
          ephemeral: true,
        });
      }
      break;

    case "send":
      const macroName = interaction.options.getString("name", true);
      const macro = await getMacroByName(macroName);

      if (!macro) {
        await interaction.reply({
          content: `❌ Macro "${macroName}" not found.`,
          ephemeral: true,
        });
        return;
      }

      // Find thread by channel ID
      const response = await fetch(`${BACKEND_URL}/threads`);
      const threads: Thread[] = (await response.json()) as Thread[];
      const thread = threads.find((t) => t.thread_id === interaction.channelId);

      if (!thread) {
        await interaction.reply({
          content: "❌ This command can only be used in a modmail thread.",
          ephemeral: true,
        });
        return;
      }

      try {
        // Send macro content to user
        const user = await client.users.fetch(thread.user_id);
        const embed = new EmbedBuilder()
          .setColor(0x0099ff)
          .setTitle("Message from Moderators")
          .setDescription(macro.content)
          .setTimestamp();

        await user.send({ embeds: [embed] });

        // Add to thread
        await addMessageToThread(
          thread.id,
          interaction.user.id,
          interaction.user.tag,
          `[MACRO: ${macroName}] ${macro.content}`
        );

        // Confirm in channel
        const confirmEmbed = new EmbedBuilder()
          .setColor(0x00ff00)
          .setAuthor({
            name: interaction.user.tag,
            iconURL: interaction.user.displayAvatarURL(),
          })
          .setDescription(`**Macro "${macroName}" sent:**\n${macro.content}`)
          .setTimestamp();

        await interaction.reply({ embeds: [confirmEmbed] });
      } catch (error) {
        console.error("Error sending macro:", error);
        await interaction.reply({
          content: "❌ Failed to send macro.",
          ephemeral: true,
        });
      }
      break;

    case "delete":
      const deleteNameParam = interaction.options.getString("name", true);

      try {
        const result = await deleteMacro(deleteNameParam);

        if (result.success) {
          await interaction.reply({
            content: `✅ Macro "${deleteNameParam}" deleted successfully.`,
            ephemeral: true,
          });
        } else {
          await interaction.reply({
            content: `❌ ${result.message}`,
            ephemeral: true,
          });
        }
      } catch (error) {
        console.error("Error deleting macro:", error);
        await interaction.reply({
          content: "❌ Failed to delete macro.",
          ephemeral: true,
        });
      }
      break;

    case "list":
      const macros = await getMacros();
      await interaction.reply({
        content: `✅ Macros: ${macros.map((m) => m.name).join(", ")}`,
        ephemeral: true,
      });
      break;

    case "edit":
      const editNameParam = interaction.options.getString("name", true);
      const editContent = interaction.options.getString("content", true);

      try {
        await editMacro(editNameParam, editContent);
        await interaction.reply({
          content: `✅ Macro "${editNameParam}" edited successfully.`,
          ephemeral: true,
        });
      } catch (error) {
        await interaction.reply({
          content: `❌ Failed to edit macro.`,
          ephemeral: true,
        });
      }
      break;

    default:
      await interaction.reply({
        content: "❌ Invalid subcommand.",
        ephemeral: true,
      });
      break;
  }
}

async function handleDeleteCommand(interaction: ChatInputCommandInteraction) {
  // delete the discord channel
  const channel = await client.channels.fetch(interaction.channelId);
  if (channel) {
    await channel.delete();
  }
}

// Handle direct messages
client.on(Events.MessageCreate, async (message) => {
  // Ignore bot messages and messages from guilds
  if (message.author.bot || message.guild) return;

  // This is a DM
  const userId = message.author.id;

  try {
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
        name: `${message.author.username}-${message.author.discriminator}`,
        type: ChannelType.GuildText,
        parent: MODMAIL_CATEGORY_ID,
        topic: `Modmail thread for ${message.author.tag} (${message.author.id})`,
      });

      // Create thread in database
      thread = await createThread(userId, channel.id);

      // Send welcome message to channel
      const welcomeEmbed = new EmbedBuilder()
        .setColor(0x0099ff)
        .setTitle("New Modmail Thread")
        .setDescription(
          `**User:** ${message.author.tag} (${
            message.author.id
          })\n**Account Created:** ${message.author.createdAt.toLocaleDateString()}\n**User Joined:** ${guild.members.cache
            .get(message.author.id)
            ?.joinedAt?.toLocaleDateString()}`
        )
        .setThumbnail(message.author.displayAvatarURL())
        .setTimestamp();

      if (channel.isTextBased() && "send" in channel) {
        await channel.send({ embeds: [welcomeEmbed] });
      }
    } else {
      channel = await client.channels.fetch(thread.thread_id);
    }

    // Add message to thread
    await addMessageToThread(
      thread.id,
      message.author.id,
      message.author.tag,
      message.content
    );

    // Forward message to modmail channel
    const messageEmbed = new EmbedBuilder()
      .setColor(0x0099ff)
      .setAuthor({
        name: message.author.tag,
        iconURL: message.author.displayAvatarURL(),
      })
      .setDescription(message.content)
      .setTimestamp();

    if (channel?.isTextBased() && "send" in channel) {
      await channel.send({ embeds: [messageEmbed] });
    }

    // Send confirmation to user
    if (!thread || thread.id === (await getThreadByUserId(userId))?.id) {
      const confirmEmbed = new EmbedBuilder()
        .setColor(0x00ff00)
        .setTitle("Message Received")
        .setDescription(
          "Your message has been sent to the moderators. They will respond as soon as possible."
        )
        .setTimestamp();

      await message.author.send({ embeds: [confirmEmbed] });
    }
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
});

// Handle messages in modmail channels (relay to user)
client.on(Events.MessageCreate, async (message) => {
  // Ignore bot messages
  if (message.author.bot) return;

  // Only handle messages in guild channels
  if (!message.guild) return;

  // Check if this is a modmail thread
  const response = await fetch(`${BACKEND_URL}/threads`);
  const threads: Thread[] = (await response.json()) as Thread[];
  const thread = threads.find((t) => t.thread_id === message.channelId);

  if (!thread || !thread.is_open) return;

  // Don't relay slash command interactions
  if (message.content.startsWith("/")) return;

  try {
    // Get the user
    const user = await client.users.fetch(thread.user_id);

    // Send message to user
    const embed = new EmbedBuilder()
      .setColor(0x0099ff)
      .setTitle("Message from Moderators")
      .setDescription(message.content)
      .setTimestamp();

    await user.send({ embeds: [embed] });

    // Add to thread
    await addMessageToThread(
      thread.id,
      message.author.id,
      message.author.tag,
      message.content
    );

    // React to confirm message was sent
    await message.react("✅");
  } catch (error) {
    console.error("Error relaying message to user:", error);
    await message.react("❌");
  }
});

// Login to Discord
client.login(DISCORD_BOT_TOKEN);
