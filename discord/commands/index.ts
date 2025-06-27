import { ChatInputCommandInteraction, Client } from "discord.js";
import { handleMessageCommand } from "./message.js";
import { handleCloseCommand } from "./close.js";
import { handleNoteCommand } from "./note.js";
import { handleBlockCommand } from "./block.js";
import { handleMacroCommand } from "./macro.js";
import { handleDeleteCommand } from "./delete.js";

export async function handleSlashCommand(
  interaction: ChatInputCommandInteraction,
  client: Client
) {
  const { commandName } = interaction;

  try {
    switch (commandName) {
      case "message":
        await handleMessageCommand(interaction, client);
        break;
      case "close":
        await handleCloseCommand(interaction, client);
        break;
      case "note":
        await handleNoteCommand(interaction);
        break;
      case "block":
        await handleBlockCommand(interaction);
        break;
      case "macro":
        await handleMacroCommand(interaction, client);
        break;
      case "delete":
        await handleDeleteCommand(interaction, client);
        break;
      default:
        await interaction.reply({
          content: "❌ Unknown command.",
          ephemeral: true,
        });
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
} 