import type { PageServerLoad, Actions } from './$types';
import { api } from '$lib/api';
import { fail } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ url }) => {
	try {
		const page = parseInt(url.searchParams.get('page') || '1');
		const limit = parseInt(url.searchParams.get('limit') || '20');

		const data = await api.getAllThreads(page, limit);
		return {
			threads: data.threads,
			pagination: data.pagination
		};
	} catch (error) {
		console.error('Error loading threads:', error);
		return {
			threads: [],
			pagination: {
				page: 1,
				limit: 20,
				total_count: 0,
				total_pages: 0,
				has_next: false,
				has_prev: false
			},
			error: 'Failed to load threads'
		};
	}
};

export const actions: Actions = {
	closeThread: async ({ request, locals: { user } }) => {
		if (!user) {
			return fail(401, { error: 'Authentication required' });
		}
		if (!user.isModerator) {
			return fail(403, { error: 'Moderator access required' });
		}

		const data = await request.formData();
		const threadId = data.get('id')?.toString();

		if (!threadId) {
			return fail(400, { error: 'Thread ID is required' });
		}

		try {
			await api.closeThread(parseInt(threadId), {
				id: user.id,
				tag: user.username
			});
			return { success: 'Thread closed successfully!' };
		} catch (error) {
			console.error('Failed to close thread:', error);
			return fail(500, { error: 'Failed to close thread' });
		}
	}
};