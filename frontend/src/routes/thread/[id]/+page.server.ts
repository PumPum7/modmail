import type { PageServerLoad, Actions } from './$types';
import { api } from '$lib/api';
import { fail, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params, locals, cookies, url }) => {
	const user = locals.user;
	const selectedGuildId = cookies.get('selected_guild_id');

	if (!user) {
		throw redirect(302, '/login');
	}

	if (!selectedGuildId) {
		throw redirect(302, '/select-server');
	}

	try {
		const page = parseInt(url.searchParams.get('page') || '1');
		const limit = parseInt(url.searchParams.get('limit') || '50');

		const data = await api.getThread(selectedGuildId, params.id, page, limit);
		const notes = await api.getThreadNotes(selectedGuildId, params.id);

		if (!data.thread || !data.messages || !data.pagination) {
			return {
				error: 'Failed to load thread',
				user
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
				error: 'Thread not found',
				user
			};
		}
		return {
			error: 'Failed to load thread',
			user
		};
	}
};

export const actions: Actions = {
	addNote: async ({ params, request, locals, cookies }) => {
		const user = locals.user;
		const selectedGuildId = cookies.get('selected_guild_id');

		if (!user) {
			return fail(401, {
				error: 'Authentication required'
			});
		}

		if (!selectedGuildId) {
			return fail(400, { error: 'No server selected' });
		}

		const data = await request.formData();
		const content = data.get('content')?.toString();

		if (!content?.trim()) {
			return fail(400, {
				error: 'Note content is required'
			});
		}

		try {
			await api.addNoteToThread(selectedGuildId, parseInt(params.id), {
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
	},

	updateUrgency: async ({ params, request, locals, cookies }) => {
		const user = locals.user;
		const selectedGuildId = cookies.get('selected_guild_id');

		if (!user) {
			return fail(401, {
				error: 'Authentication required'
			});
		}

		if (!user?.isModerator) {
			return fail(403, {
				error: 'Moderator access required'
			});
		}

		if (!selectedGuildId) {
			return fail(400, { error: 'No server selected' });
		}

		const data = await request.formData();
		const urgency = data.get('urgency')?.toString();

		if (!urgency) {
			return fail(400, {
				error: 'Urgency level is required'
			});
		}

		const validUrgencies = ['Low', 'Medium', 'High', 'Urgent'];
		if (!validUrgencies.includes(urgency)) {
			return fail(400, {
				error: 'Invalid urgency level'
			});
		}

		try {
			await api.updateThreadUrgency(selectedGuildId, parseInt(params.id), urgency);

			return {
				success: 'Thread urgency updated successfully!'
			};
		} catch (error) {
			console.error('Error updating urgency:', error);
			return fail(500, {
				error: 'Failed to update thread urgency'
			});
		}
	},

	closeThread: async ({ params, locals, cookies }) => {
		const user = locals.user;
		const selectedGuildId = cookies.get('selected_guild_id');

		if (!user) {
			return fail(401, { error: 'Authentication required' });
		}
		if (!user?.isModerator) {
			return fail(403, { error: 'Moderator access required' });
		}
		if (!selectedGuildId) {
			return fail(400, { error: 'No server selected' });
		}

		try {
			await api.closeThread(selectedGuildId, parseInt(params.id), {
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
