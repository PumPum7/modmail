import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { parseJWT } from '$lib/auth';
import { PUBLIC_BACKEND_URL } from '$env/static/public';

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
		const guildsResponse = await fetch('https://discord.com/api/users/@me/guilds', {
			headers: {
				Authorization: `Bearer ${user.access_token}`
			}
		});

		if (!guildsResponse.ok) {
			return json({ error: 'Failed to fetch guilds from Discord' }, { status: 500 });
		}

		const guilds = await guildsResponse.json();

		const formattedGuilds = guilds
			.filter(
				(guild: any) => (guild.permissions & 0x8) == 0x8 || (guild.permissions & 0x20) == 0x20
			)
			.map((guild: any) => ({
				guild_id: guild.id,
				guild_name: guild.name,
				guild_icon: guild.icon,
				user_has_permissions: true
			}));

		// Filter out guilds that the bot is not in
		const validationResponse = await fetch(`${PUBLIC_BACKEND_URL}/validate-guilds`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(formattedGuilds)
		});

		if (!validationResponse.ok) {
			return json({ error: 'Failed to validate guilds with backend' }, { status: 500 });
		}

		const validatedGuilds = await validationResponse.json();

		return json(validatedGuilds);
	} catch (error) {
		console.error('Error fetching user guilds:', error);
		return json({ error: 'Internal server error' }, { status: 500 });
	}
};
