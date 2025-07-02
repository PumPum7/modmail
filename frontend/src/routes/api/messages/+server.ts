import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ fetch, cookies }) => {
	const selectedGuildId = cookies.get('selected_guild_id');

	if (!selectedGuildId) {
		return json({ error: 'No server selected' }, { status: 400 });
	}

	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/guilds/${selectedGuildId}/messages`);
		if (!response.ok) {
			throw new Error('Failed to fetch messages');
		}
		const messages = await response.json();
		return json(messages);
	} catch (error) {
		console.error('Error fetching messages:', error);
		return json({ error: 'Failed to fetch messages' }, { status: 500 });
	}
};
