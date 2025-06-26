import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/messages`);
		if (!response.ok) {
			throw new Error('Failed to fetch messages');
		}
		const messages = await response.json();
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