import { ButtonInteraction, Client } from "discord.js";
import { getThreadByChannelId, addMessageToThread, getMacroByName } from "../api.js";
import { createModeratorMessageEmbed, createConfirmationEmbed } from "../utils.js";

export async function handleButtonInteraction(interaction: ButtonInteraction, client: Client) {
  if (!interaction.customId.startsWith('quick_reply_')) return;
  
  const macroName = interaction.customId.replace('quick_reply_', '');
  const thread = await getThreadByChannelId(interaction.channelId);

  if (!thread) {
    await interaction.reply({
      content: "❌ This button can only be used in a modmail thread.",
      ephemeral: true,
    });
    return;
  }

  try {
    const macro = await getMacroByName(macroName);
    
    if (!macro) {
      await interaction.reply({
        content: `❌ Macro "${macroName}" not found.`,
        ephemeral: true,
      });
      return;
    }

    // Send macro content to user
    const user = await client.users.fetch(thread.user_id);
    const embed = createModeratorMessageEmbed(macro.content);
    await user.send({ embeds: [embed] });

    // Add to thread
    await addMessageToThread(
      thread.id,
      interaction.user.id,
      interaction.user.tag,
      `[QUICK REPLY: ${macroName}] ${macro.content}`
    );

    // Confirm in channel
    const confirmEmbed = createConfirmationEmbed(
      user, 
      macro.content, 
      `Quick reply "${macroName}" sent to`
    );

    await interaction.reply({ embeds: [confirmEmbed] });
  } catch (error) {
    console.error("Error sending quick reply:", error);
    await interaction.reply({
      content: "❌ Failed to send quick reply.",
      ephemeral: true,
    });
  }
}
