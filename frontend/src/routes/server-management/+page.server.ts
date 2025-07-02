import type { PageServerLoad } from './$types';
import { api } from '$lib/api';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ locals, cookies }) => {
	const user = locals.user;
	const selectedGuildId = cookies.get('selected_guild_id');

	if (!user) {
		throw redirect(302, '/login');
	}

	if (!selectedGuildId) {
		throw redirect(302, '/select-server');
	}

	try {
		// Get basic analytics for server overview
		const overview = await api.getAnalyticsOverview(selectedGuildId);

		return {
			overview,
			user,
			selectedGuildId
		};
	} catch (error) {
		console.error('Error loading server management data:', error);
		return {
			overview: null,
			user,
			selectedGuildId,
			error: 'Failed to load server data'
		};
	}
};
