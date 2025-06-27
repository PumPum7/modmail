import { ChatInputCommandInteraction, MessageFlagsBitField } from "discord.js";
import { getThreadByChannelId, addNoteToThread } from "../api.js";

export async function handleNoteCommand(interaction: ChatInputCommandInteraction) {
  const noteContent = interaction.options.getString("content", true);
  const channelId = interaction.channelId;

  const thread = await getThreadByChannelId(channelId);

  if (!thread) {
    await interaction.reply({
      content: "❌ This command can only be used in a modmail thread.",
      flags: MessageFlagsBitField.Flags.Ephemeral,
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
      flags: MessageFlagsBitField.Flags.Ephemeral,
    });
  } catch (error) {
    console.error("Error adding note:", error);
    await interaction.reply({
      content: "❌ Failed to add note.",
      flags: MessageFlagsBitField.Flags.Ephemeral,
    });
  }
} 