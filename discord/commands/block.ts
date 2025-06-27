import { ChatInputCommandInteraction } from "discord.js";
import { blockUser, isUserBlocked } from "../api.js";

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
        ephemeral: true,
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
      ephemeral: true,
    });
  }
} 