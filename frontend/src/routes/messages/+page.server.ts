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
		const messages = await api.getAllMessages(selectedGuildId);
		return {
			messages,
			user
		};
	} catch (error) {
		console.error('Error loading messages:', error);
		return {
			messages: [],
			error: 'Failed to load messages',
			user
		};
	}
};
