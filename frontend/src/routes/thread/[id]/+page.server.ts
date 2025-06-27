import type { PageServerLoad, Actions } from './$types';
import { api } from '$lib/api';
import { fail } from '@sveltejs/kit';

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

export const actions: Actions = {
	addNote: async ({ params, request, locals: { user } }) => {
		if (!user) {
			return fail(401, {
				error: 'Authentication required'
			});
		}

		const data = await request.formData();
		const content = data.get('content')?.toString();

		if (!content?.trim()) {
			return fail(400, {
				error: 'Note content is required'
			});
		}

		try {
			await api.addNoteToThread(parseInt(params.id), {
				author_id: user.id,
				author_tag: user.username,
				content: content.trim()
			});

			return {
				success: 'Internal note added successfully!'
			};
		} catch (error) {
			console.error('Error adding note:', error);
			return fail(500, {
				error: 'Failed to add note'
			});
		}
	}
};
