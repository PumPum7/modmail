import { redirect } from '@sveltejs/kit';
import { api } from '$lib/api';
import { parseJWT, isGuildModerator } from '$lib/auth';
import type { LayoutServerLoad } from './$types';
import { PUBLIC_BACKEND_URL } from '$env/static/public';

export const load: LayoutServerLoad = async ({ locals, cookies, url, fetch }) => {
	const user = locals.user;
	const selectedGuildId = cookies.get('selected_guild_id');

	// If user is authenticated but no guild is selected, redirect to server selection
	if (user && !selectedGuildId && url.pathname !== '/select-server' && url.pathname !== '/login') {
		throw redirect(302, '/select-server');
	}

        // If user is not authenticated and trying to access protected routes, redirect to login
        if (
                !user &&
                url.pathname !== '/login' &&
                url.pathname !== '/select-server' &&
                url.pathname !== '/'
        ) {
                throw redirect(302, '/login');
        }

	let currentGuild = null;
	let availableGuilds = [];
	let userWithGuildInfo = user;

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
							guild_icon: guild.icon,
							user_has_permissions: true
						}));

						const validationResponse = await fetch(`${PUBLIC_BACKEND_URL}/validate-guilds`, {
							method: 'POST',
							headers: {
								'Content-Type': 'application/json'
							},
							body: JSON.stringify(availableGuilds)
						});

						if (!validationResponse.ok) {
							throw new Error('Failed to validate guilds with backend');
						}

						const validatedGuilds = await validationResponse.json();

						// Find current guild info
						currentGuild = validatedGuilds.find((guild: any) => guild.guild_id === selectedGuildId);

						// Get user's member info for the selected guild to check moderator status
						try {
							const memberResponse = await fetch(
								`https://discord.com/api/users/@me/guilds/${selectedGuildId}/member`,
								{
									headers: {
										Authorization: `Bearer ${tokenUser.access_token}`
									}
								}
							);

							if (memberResponse.ok) {
								const member = await memberResponse.json();
								const isModerator = await isGuildModerator(
									selectedGuildId,
									member.roles,
									tokenUser.access_token,
									fetch
								);

								userWithGuildInfo = {
									...user,
									isModerator
								};
							}
						} catch (memberError) {
							console.error('Error fetching guild member info:', memberError);
							// Fall back to no moderator privileges if we can't verify
							userWithGuildInfo = {
								...user,
								isModerator: false
							};
						}
					}
				}
			}
		} catch (error) {
			console.error('Error fetching guild information:', error);
		}
	}

	return {
		user: userWithGuildInfo || null,
		selectedGuildId: selectedGuildId || null,
		currentGuild,
		availableGuilds
	};
};
