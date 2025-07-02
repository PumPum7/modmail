import { redirect } from '@sveltejs/kit';
import { api } from '$lib/api';
import { parseJWT } from '$lib/auth';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals, cookies, url, fetch }) => {
	const user = locals.user;
	const selectedGuildId = cookies.get('selected_guild_id');

	// If user is authenticated but no guild is selected, redirect to server selection
	if (user && !selectedGuildId && url.pathname !== '/select-server' && url.pathname !== '/login') {
		throw redirect(302, '/select-server');
	}

	// If user is not authenticated and trying to access protected routes, redirect to login
	if (!user && url.pathname !== '/login' && url.pathname !== '/select-server') {
		throw redirect(302, '/login');
	}

	let currentGuild = null;
	let availableGuilds = [];

	// Fetch current guild info and available guilds if user is authenticated
	if (user && selectedGuildId) {
		try {
			const authToken = cookies.get('auth_token');
			if (authToken) {
				const tokenUser = parseJWT(authToken);
				if (tokenUser?.access_token) {
					// Get user's guilds from Discord API
					const response = await fetch('https://discord.com/api/users/@me/guilds', {
						headers: {
							Authorization: `Bearer ${tokenUser.access_token}`
						}
					});

					if (response.ok) {
						const guilds = await response.json();
						availableGuilds = guilds.map((guild: any) => ({
							guild_id: guild.id,
							guild_name: guild.name,
							guild_icon: guild.icon
						}));

						// Find current guild info
						currentGuild = availableGuilds.find((guild: any) => guild.guild_id === selectedGuildId);
					}
				}
			}
		} catch (error) {
			console.error('Error fetching guild information:', error);
		}
	}

	return {
		user: user || null,
		selectedGuildId: selectedGuildId || null,
		currentGuild,
		availableGuilds
	};
};
