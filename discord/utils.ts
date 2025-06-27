import { EmbedBuilder, User, Guild, Client, AttachmentBuilder } from "discord.js";
import type { Attachment } from "./types.js";

const RANDOMIZE_NAMES = process.env.RANDOMIZE_NAMES === "true";
const FRONTEND_URL = process.env.PUBLIC_FRONT_END_URL;

export function generateRandomString(): string {
  return (
    Math.random().toString(36).substring(2, 15) +
    Math.random().toString(36).substring(2, 15)
  );
}

export function generateWelcomeEmbed(user: User, guild: Guild): EmbedBuilder {
  const member = guild.members.cache.get(user.id);
  return new EmbedBuilder()
    .setColor(0x0099ff)
    .setTitle("New Modmail Thread")
    .setDescription(
      `**User:** ${user.tag} (${
        user.id
      })\n**Account Created:** ${user.createdAt.toLocaleDateString()}\n**User Joined:** ${member?.joinedAt?.toLocaleDateString()}`
    )
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