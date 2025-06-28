import { EmbedBuilder, User, Guild, Client, AttachmentBuilder, ActionRowBuilder, ButtonBuilder, ButtonStyle, ModalBuilder, TextInputBuilder, TextInputStyle } from "discord.js";
import type { Attachment } from "./types.js";

const RANDOMIZE_NAMES = process.env.RANDOMIZE_NAMES === "true";
const FRONTEND_URL = process.env.PUBLIC_FRONT_END_URL;

export function generateRandomString(): string {
  return (
    Math.random().toString(36).substring(2, 15) +
    Math.random().toString(36).substring(2, 15)
  );
}

export function generateWelcomeEmbed(user: User, guild: Guild, introData?: any): EmbedBuilder {
  const member = guild.members.cache.get(user.id);
  let description = `**User:** ${user.tag} (${
    user.id
  })\n**Account Created:** ${user.createdAt.toLocaleDateString()}\n**User Joined:** ${member?.joinedAt?.toLocaleDateString()}`;
  
  // Add intro data if available
  if (introData) {
    description += `\n\n**User Introduction:**`;
    if (introData.subject) description += `\n**Subject:** ${introData.subject}`;
    if (introData.description) description += `\n**Description:** ${introData.description}`;
    if (introData.urgency) description += `\n**Priority:** ${introData.urgency}`;
  }
  
  return new EmbedBuilder()
    .setColor(0x0099ff)
    .setTitle("New Modmail Thread")
    .setDescription(description)
    .setThumbnail(user.displayAvatarURL());
}

export function createModeratorMessageEmbed(content: string): EmbedBuilder {
  return new EmbedBuilder()
    .setColor(0x0099ff)
    .setTitle("Message from Moderators")
    .setDescription(content)
    .setTimestamp();
}

export function createUserMessageEmbed(user: User, content: string): EmbedBuilder {
  return new EmbedBuilder()
    .setColor(0x0099ff)
    .setAuthor({
      name: user.tag,
      iconURL: user.displayAvatarURL(),
    })
    .setDescription(content || "*No text content*")
    .setTimestamp();
}

export function createConfirmationEmbed(user: User, content: string, prefix: string = "Message sent to"): EmbedBuilder {
  return new EmbedBuilder()
    .setColor(0x00ff00)
    .setAuthor({
      name: user.tag,
      iconURL: user.displayAvatarURL(),
    })
    .setDescription(`**${prefix} ${user.tag}:**\n${content}`)
    .setTimestamp();
}

export function createThreadClosedEmbed(closedBy: User): EmbedBuilder {
  return new EmbedBuilder()
    .setColor(0xff0000)
    .setTitle("Thread Closed")
    .setDescription(`This thread has been closed by ${closedBy.tag}`)
    .setTimestamp();
}

export function createLogEmbed(user: User, closedBy: User, threadId: number): EmbedBuilder {
  const threadUrl = `${FRONTEND_URL}/thread/${threadId}`;
  
  return new EmbedBuilder()
    .setColor(0xff0000)
    .setTitle("Thread Closed")
    .setDescription(
      `**User:** ${user.tag} (${user.id})\n**Closed by:** ${closedBy.tag}\n**Thread:** [View Thread](${threadUrl})`
    )
    .setThumbnail(user.displayAvatarURL())
    .setTimestamp();
}

export function processAttachments(messageAttachments: any[]): Attachment[] {
  return messageAttachments.map((attachment) => ({
    url: attachment.url,
    filename: attachment.name,
    content_type: attachment.contentType || "unknown",
    size: attachment.size,
  }));
}

export function categorizeAttachments(attachments: Attachment[]) {
  const imageAttachments = attachments.filter((att) =>
    att.content_type?.startsWith("image/")
  );
  const nonImageAttachments = attachments.filter(
    (att) => !att.content_type?.startsWith("image/")
  );
  
  return { imageAttachments, nonImageAttachments };
}

export function addAttachmentsToEmbed(embed: EmbedBuilder, attachments: Attachment[]) {
  const { imageAttachments, nonImageAttachments } = categorizeAttachments(attachments);
  
  if (imageAttachments.length > 0) {
    embed.setImage(imageAttachments[0]?.url || null);
  }

  if (nonImageAttachments.length > 0) {
    embed.addFields({
      name: "Attachments",
      value: nonImageAttachments
        .map((att) => `[Attachment: ${att.filename}]`)
        .join("\n"),
    });
  }
  
  return embed;
}

export function generateChannelName(user: User): string {
  return RANDOMIZE_NAMES
    ? generateRandomString()
    : `${user.username}-${user.discriminator}`;
}

export function createUserClosureNotificationEmbed(): EmbedBuilder {
  return new EmbedBuilder()
    .setColor(0xff0000)
    .setTitle("Modmail Thread Closed")
    .setDescription("Your modmail thread has been closed by the moderators.")
    .setTimestamp();
}

export function createUserConfirmationEmbed(): EmbedBuilder {
  return new EmbedBuilder()
    .setColor(0x00ff00)
    .setTitle("Message Received")
    .setDescription(
      "Your message has been sent to the moderators. They will respond as soon as possible."
    )
    .setTimestamp();
} 

export function createQuickReplyButtons(macros: any[]) {
  if (macros.length === 0) return [];
  
  const buttons = macros.slice(0, 3).map((macro, index) => 
    new ButtonBuilder()
      .setCustomId(`quick_reply_${macro.name}`)
      .setLabel(macro.name)
      .setStyle(ButtonStyle.Primary)
  );
  
  const row = new ActionRowBuilder<ButtonBuilder>().addComponents(buttons);
  return [row];
}

export function createIntroPromptEmbed(): EmbedBuilder {
  return new EmbedBuilder()
    .setColor(0x0099ff)
    .setTitle("Welcome to Modmail")
    .setDescription(
      "Thank you for contacting our moderation team! Before we begin, please click the button below to provide some details about your inquiry. This helps our moderators assist you more effectively."
    )
    .addFields({
      name: "What happens next?",
      value: "• Fill out a quick form with your inquiry details\n• Your message will be forwarded to our moderation team\n• A moderator will respond to you as soon as possible",
      inline: false
    })
    .setFooter({ text: "Click 'Start Conversation' to begin" });
}

export function createIntroModal(userId: string): ModalBuilder {
  const modal = new ModalBuilder()
    .setCustomId(`intro_modal_${userId}`)
    .setTitle("Contact Information");

  const subjectInput = new TextInputBuilder()
    .setCustomId("subject")
    .setLabel("Subject/Topic")
    .setStyle(TextInputStyle.Short)
    .setPlaceholder("Brief summary of your inquiry...")
    .setRequired(true)
    .setMaxLength(100);

  const descriptionInput = new TextInputBuilder()
    .setCustomId("description")
    .setLabel("Detailed Description")
    .setStyle(TextInputStyle.Paragraph)
    .setPlaceholder("Please provide more details about your inquiry...")
    .setRequired(true)
    .setMaxLength(1000);

  const urgencyInput = new TextInputBuilder()
    .setCustomId("urgency")
    .setLabel("Priority Level")
    .setStyle(TextInputStyle.Short)
    .setPlaceholder("Low, Medium, High, or Urgent")
    .setRequired(false)
    .setMaxLength(20);

  const subjectRow = new ActionRowBuilder<TextInputBuilder>().addComponents(subjectInput);
  const descriptionRow = new ActionRowBuilder<TextInputBuilder>().addComponents(descriptionInput);
  const urgencyRow = new ActionRowBuilder<TextInputBuilder>().addComponents(urgencyInput);

  modal.addComponents(subjectRow, descriptionRow, urgencyRow);

  return modal;
}