import type { PageServerLoad } from './$types';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ locals: { user } }) => {
	try {
		const [overview, threadVolume, moderatorActivity, responseTimes] = await Promise.all([
			api.getAnalyticsOverview(),
			api.getThreadVolume(),
			api.getModeratorActivity(),
			api.getResponseTimes()
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
