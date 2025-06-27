import type { PageServerLoad } from './$types';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ locals: { user } }) => {
	try {
		const messages = await api.getAllMessages();
		return {
			messages
		};
	} catch (error) {
		console.error('Error loading messages:', error);
		return {
			messages: [],
			error: 'Failed to load messages'
		};
	}
};
