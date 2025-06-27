import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, url }) => {
	try {
		const page = parseInt(url.searchParams.get('page') || '1');
		const limit = parseInt(url.searchParams.get('limit') || '20');

		const response = await fetch(`${PUBLIC_BACKEND_URL}/threads?page=${page}&limit=${limit}`);
		if (!response.ok) {
			throw new Error('Failed to fetch threads');
		}
		const data = await response.json();
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
