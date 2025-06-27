import { ChatInputCommandInteraction, MessageFlagsBitField } from "discord.js";
import { blockUser, isUserBlocked, unblockUser } from "../api.js";

export async function handleBlockCommand(interaction: ChatInputCommandInteraction) {
  const user = interaction.options.getUser("user", true);
  const reason =
    interaction.options.getString("reason") || "No reason provided";

  try {
    // Check if user is already blocked
    const isBlocked = await isUserBlocked(user.id);
    if (isBlocked) {
      await interaction.reply({
        content: `❌ User ${user.tag} is already blocked.`,
        flags: MessageFlagsBitField.Flags.Ephemeral
      });
      return;
    }

    // Block the user
    await blockUser(
      user.id,
      user.tag,
      interaction.user.id,
      interaction.user.tag,
      reason
    );

    await interaction.reply({
      content: `✅ User ${user.tag} has been blocked. Reason: ${reason}`,
      ephemeral: true,
    });
  } catch (error) {
    console.error("Error blocking user:", error);
    await interaction.reply({
      content: "❌ Failed to block user.",
        flags: MessageFlagsBitField.Flags.Ephemeral
    });
  }
} 

export async function handleUnblockCommand(interaction: ChatInputCommandInteraction) {
  const user = interaction.options.getUser("user", true);

  try {
    const isBlocked = await isUserBlocked(user.id);
    if (!isBlocked) {
      await interaction.reply({
        content: `❌ User ${user.tag} is not blocked.`,
        flags: MessageFlagsBitField.Flags.Ephemeral
      });
      return;
    }

    await unblockUser(user.id);

    await interaction.reply({
      content: `✅ User ${user.tag} has been unblocked.`,
      flags: MessageFlagsBitField.Flags.Ephemeral
    });
  } catch (error) {
    console.error("Error unblocking user:", error);
    await interaction.reply({
      content: "❌ Failed to unblock user.",
      flags: MessageFlagsBitField.Flags.Ephemeral
    });
  }
}