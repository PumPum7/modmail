import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { PageServerLoad } from './$types';
import type { Message, Thread } from '$lib/api';

export const load: PageServerLoad = async ({ params, fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/threads/${params.id}`);
		if (!response.ok) {
			if (response.status === 404) {
				return {
					error: 'Thread not found'
				};
			}
			return {
				error: 'Failed to fetch thread'
			};
		}
		const [thread, messages]: [Thread, Message[]] = await response.json();
		return {
			thread,
			messages
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
