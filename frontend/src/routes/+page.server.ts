import type { PageServerLoad, Actions } from './$types';
import { api } from '$lib/api';
import { fail, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ locals, cookies, url }) => {
        const user = locals.user;
        const selectedGuildId = cookies.get('selected_guild_id');

        // If the user isn't logged in simply return empty data so the
        // page can render the marketing homepage.
        if (!user) {
                return { threads: [], pagination: null, user: null };
        }

        if (!selectedGuildId) {
                throw redirect(302, '/select-server');
        }

	try {
		const page = parseInt(url.searchParams.get('page') || '1');
		const limit = parseInt(url.searchParams.get('limit') || '20');

		const threadsResponse = await api.getAllThreads(selectedGuildId, page, limit);

		return {
			threads: threadsResponse.threads,
			pagination: threadsResponse.pagination,
			user
		};
	} catch (error) {
		console.error('Error loading threads:', error);
		return {
			threads: [],
			pagination: null,
			error: 'Failed to load threads',
			user
		};
	}
};

export const actions: Actions = {
	closeThread: async ({ request, locals, cookies }) => {
		const user = locals.user;
		const selectedGuildId = cookies.get('selected_guild_id');

		if (!user || !selectedGuildId) {
			return { error: 'Not authenticated or no server selected' };
		}

		try {
			const data = await request.formData();
			const threadId = parseInt(data.get('id') as string);

			await api.closeThread(selectedGuildId, threadId, {
				id: user.id,
				tag: `${user.username}#${user.discriminator}`
			});

			return { success: 'Thread closed successfully' };
		} catch (error) {
			console.error('Error closing thread:', error);
			return { error: 'Failed to close thread' };
		}
	}
};
