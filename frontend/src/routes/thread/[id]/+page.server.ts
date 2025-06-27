import type { PageServerLoad } from './$types';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ params, locals: { user }, url }) => {
	try {
		const page = parseInt(url.searchParams.get('page') || '1');
		const limit = parseInt(url.searchParams.get('limit') || '50');

		const data = await api.getThread(params.id, page, limit);

		const notes = await api.getThreadNotes(params.id);

		if (!data.thread || !data.messages || !data.pagination) {
			return {
				error: 'Failed to load thread'
			};
		}

		return {
			thread: data.thread,
			messages: data.messages,
			pagination: data.pagination,
			notes,
			user
		};
	} catch (err) {
		console.error('Error loading thread:', err);
		if (err instanceof Error && err.message.includes('404')) {
			return {
				error: 'Thread not found'
			};
		}
		return {
			error: 'Failed to load thread'
		};
	}
};
