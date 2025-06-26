import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/threads`);
		if (!response.ok) {
			throw new Error('Failed to fetch threads');
		}
		const threads = await response.json();
		return {
			threads
		};
	} catch (error) {
		console.error('Error loading threads:', error);
		return {
			threads: [],
			error: 'Failed to load threads'
		};
	}
}; 