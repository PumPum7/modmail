import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { parseJWT, isModerator } from '$lib/auth';
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

		// Check permissions for each guild
		const guildsWithPermissions = [];
		for (const guild of guilds) {
			// Check if user has administrator or manage guild permissions
			const hasManagePermissions =
				(guild.permissions & 0x20) === 0x20 || // MANAGE_GUILD
				(guild.permissions & 0x8) === 0x8; // ADMINISTRATOR

			if (hasManagePermissions) {
				// Get member info to check specific roles
				try {
					const memberResponse = await fetch(
						`https://discord.com/api/users/@me/guilds/${guild.id}/member`,
						{
							headers: {
								Authorization: `Bearer ${user.access_token}`
							}
						}
					);

					console.log(memberResponse);

					if (memberResponse.ok) {
						const member = await memberResponse.json();

						// Try to get guild config to check guild-specific moderator roles
						let userIsModerator = false;
						try {
							const configResponse = await fetch(`${PUBLIC_BACKEND_URL}/guilds/${guild.id}/config`);
							if (configResponse.ok) {
								const guildConfig = await configResponse.json();
								const moderatorRoleIds = guildConfig.moderator_role_ids || [];
								userIsModerator = isModerator(member.roles, moderatorRoleIds);
							}
						} catch (configError) {
							console.log(
								`No guild config found for ${guild.id}, checking manage permissions only`
							);
						}

						if (userIsModerator || hasManagePermissions) {
							guildsWithPermissions.push({
								guild_id: guild.id,
								guild_name: guild.name,
								guild_icon: guild.icon,
								user_has_permissions: true
							});
						}
					} else if (hasManagePermissions) {
						// Fallback: if we can't get member info but user has manage permissions
						guildsWithPermissions.push({
							guild_id: guild.id,
							guild_name: guild.name,
							guild_icon: guild.icon,
							user_has_permissions: true
						});
					}
				} catch (error) {
					console.error(`Error checking member for guild ${guild.id}:`, error);
					// Fallback to permission-based check
					if (hasManagePermissions) {
						guildsWithPermissions.push({
							guild_id: guild.id,
							guild_name: guild.name,
							guild_icon: guild.icon,
							user_has_permissions: true
						});
					}
				}
			}
		}

		// Validate guilds with backend (check if bot is present and configured)
		if (guildsWithPermissions.length > 0) {
			try {
				const validationResponse = await fetch(`${PUBLIC_BACKEND_URL}/validate-guilds`, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify(guildsWithPermissions)
				});

				if (validationResponse.ok) {
					const validatedGuilds = await validationResponse.json();
					// Only return guilds where bot is present and configured
					const availableGuilds = validatedGuilds
						.filter((guild: any) => guild.has_bot)
						.map((guild: any) => ({
							guild_id: guild.guild_id,
							guild_name: guild.guild_name,
							guild_icon: guild.guild_icon
						}));

					return json(availableGuilds);
				}
			} catch (error) {
				console.error('Error validating guilds with backend:', error);
			}
		}

		// Fallback: return empty array if validation fails
		return json([]);
	} catch (error) {
		console.error('Error fetching user guilds:', error);
		return json({ error: 'Internal server error' }, { status: 500 });
	}
};
