import { redirect } from '@sveltejs/kit';
import { api } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies }) => {
	const authToken = cookies.get('auth_token');

	if (!authToken) {
		throw redirect(302, '/login');
	}

	try {
		const guilds = await api.getUserGuilds();

		if (guilds.length === 0) {
			return {
				error: 'You are not a moderator in any servers with this modmail bot configured.'
			};
		}

		// If user is only in one server, auto-select it
		if (guilds.length === 1) {
			const guildId = guilds[0].guild_id;
			cookies.set('selected_guild_id', guildId, {
				path: '/',
				httpOnly: true,
				secure: process.env.NODE_ENV === 'production',
				sameSite: 'lax',
				maxAge: 60 * 60 * 24 * 30 // 30 days
			});
			throw redirect(302, '/');
		}

		return {
			guilds
		};
	} catch (error) {
		console.error('Error loading server selection:', error);
		return {
			error: 'Failed to load available servers. Please try logging in again.'
		};
	}
};

export const actions = {
	selectServer: async ({ request, cookies }) => {
		const data = await request.formData();
		const guildId = data.get('guild_id') as string;

		if (!guildId) {
			return {
				error: 'Please select a server.'
			};
		}

		// Set selected guild cookie
		cookies.set('selected_guild_id', guildId, {
			path: '/',
			httpOnly: true,
			secure: process.env.NODE_ENV === 'production',
			sameSite: 'lax',
			maxAge: 60 * 60 * 24 * 30 // 30 days
		});

		throw redirect(302, '/');
	}
};
