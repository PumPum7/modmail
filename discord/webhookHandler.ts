import { Client, User } from 'discord.js';
import {
	createThreadClosedEmbed,
	createLogEmbed,
	createUserClosureNotificationEmbed,
} from './utils.js';

const LOG_CHANNEL_ID = process.env.PUBLIC_LOG_CHANNEL;
const FRONTEND_URL = process.env.PUBLIC_FRONT_END_URL;

export async function handleWebhookThreadClosed(payload: any, client: Client) {
	try {
		const { thread, closed_by_id, closed_by_tag } = payload;

		// Create mock user object for the moderator who closed the thread
		const closedByUser = {
			id: closed_by_id,
			tag: closed_by_tag,
			username: closed_by_tag.split('#')[0] || closed_by_tag,
		};

		// Post to log channel if configured
		if (LOG_CHANNEL_ID && FRONTEND_URL) {
			try {
				const logChannel = await client.channels.fetch(LOG_CHANNEL_ID);
				if (logChannel?.isTextBased() && 'send' in logChannel) {
					const user = await client.users.fetch(thread.user_id);
					// we pass all user properties we need
					const logEmbed = createLogEmbed(user, closedByUser as unknown as User, thread.id);
					await logChannel.send({ embeds: [logEmbed] });
				}
			} catch (error) {
				console.error('Error posting to log channel:', error);
			}
		}

		// Notify user
		try {
			const user = await client.users.fetch(thread.user_id);
			const userEmbed = createUserClosureNotificationEmbed();
			await user.send({ embeds: [userEmbed] });
		} catch (error) {
			console.error('Error notifying user of closure:', error);
		}

		console.log(`Thread ${thread.id} closed by ${closed_by_tag} via web interface`);
	} catch (error) {
		console.error('Error handling webhook thread closure:', error);
	}
}
