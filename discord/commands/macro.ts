import { ChatInputCommandInteraction, Client } from "discord.js";
import { 
  createMacro, 
  getMacroByName, 
  deleteMacro, 
  getMacros, 
  editMacro,
  getThreadByChannelId,
  addMessageToThread
} from "../api.js";
import { createModeratorMessageEmbed, createConfirmationEmbed } from "../utils.js";

export async function handleMacroCommand(
  interaction: ChatInputCommandInteraction,
  client: Client
) {
  const subcommand = interaction.options.getSubcommand();

  switch (subcommand) {
    case "create":
      await handleMacroCreate(interaction);
      break;
    case "send":
      await handleMacroSend(interaction, client);
      break;
    case "delete":
      await handleMacroDelete(interaction);
      break;
    case "list":
      await handleMacroList(interaction);
      break;
    case "edit":
      await handleMacroEdit(interaction);
      break;
    default:
      await interaction.reply({
        content: "❌ Invalid subcommand.",
        ephemeral: true,
      });
      break;
  }
}

async function handleMacroCreate(interaction: ChatInputCommandInteraction) {
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
}

async function handleMacroSend(interaction: ChatInputCommandInteraction, client: Client) {
  const macroName = interaction.options.getString("name", true);
  const macro = await getMacroByName(macroName);

  if (!macro) {
    await interaction.reply({
      content: `❌ Macro "${macroName}" not found.`,
      ephemeral: true,
    });
    return;
  }

  const thread = await getThreadByChannelId(interaction.channelId);

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
    const embed = createModeratorMessageEmbed(macro.content);
    await user.send({ embeds: [embed] });

    // Add to thread
    await addMessageToThread(
      thread.id,
      interaction.user.id,
      interaction.user.tag,
      `[MACRO: ${macroName}] ${macro.content}`
    );

    // Confirm in channel
    const confirmEmbed = createConfirmationEmbed(
      user, 
      macro.content, 
      `Macro "${macroName}" sent to`
    );

    await interaction.reply({ embeds: [confirmEmbed] });
  } catch (error) {
    console.error("Error sending macro:", error);
    await interaction.reply({
      content: "❌ Failed to send macro.",
      ephemeral: true,
    });
  }
}

async function handleMacroDelete(interaction: ChatInputCommandInteraction) {
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
}

async function handleMacroList(interaction: ChatInputCommandInteraction) {
  try {
    const macros = await getMacros();
    await interaction.reply({
      content: `✅ Macros: ${macros.map((m) => m.name).join(", ")}`,
      ephemeral: true,
    });
  } catch (error) {
    console.error("Error listing macros:", error);
    await interaction.reply({
      content: "❌ Failed to list macros.",
      ephemeral: true,
    });
  }
}

async function handleMacroEdit(interaction: ChatInputCommandInteraction) {
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
} 