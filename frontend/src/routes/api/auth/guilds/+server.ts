import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { parseJWT } from '$lib/auth';

export const GET: RequestHandler = async ({ cookies, fetch }) => {
	const authToken = cookies.get('auth_token');

	if (!authToken) {
		return json({ error: 'Not authenticated' }, { status: 401 });
	}

	try {
		const user = parseJWT(authToken);
		if (!user || !user.access_token) {
			return json({ error: 'Invalid token' }, { status: 401 });
		}

		// Get user's guilds from Discord API
		const response = await fetch('https://discord.com/api/users/@me/guilds', {
			headers: {
				Authorization: `Bearer ${user.access_token}`
			}
		});

		if (!response.ok) {
			return json({ error: 'Failed to fetch guilds from Discord' }, { status: 500 });
		}

		const guilds = await response.json();

		// Filter guilds where user has moderator permissions
		// For now, return all guilds - in production you'd check permissions
		const availableGuilds = guilds.map((guild: any) => ({
			guild_id: guild.id,
			guild_name: guild.name,
			guild_icon: guild.icon
		}));

		return json(availableGuilds);
	} catch (error) {
		console.error('Error fetching user guilds:', error);
		return json({ error: 'Internal server error' }, { status: 500 });
	}
};
