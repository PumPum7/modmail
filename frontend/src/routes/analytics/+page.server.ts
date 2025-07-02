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
		const [overview, threadVolume, moderatorActivity, responseTimes] = await Promise.all([
			api.getAnalyticsOverview(selectedGuildId),
			api.getThreadVolume(selectedGuildId),
			api.getModeratorActivity(selectedGuildId),
			api.getResponseTimes(selectedGuildId)
		]);

		return {
			overview,
			threadVolume,
			moderatorActivity,
			responseTimes,
			user
		};
	} catch (error) {
		console.error('Error loading analytics:', error);
		return {
			overview: null,
			threadVolume: [],
			moderatorActivity: [],
			responseTimes: null,
			user,
			error: 'Failed to load analytics data'
		};
	}
};
