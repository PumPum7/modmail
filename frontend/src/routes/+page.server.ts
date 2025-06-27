import type { PageServerLoad } from './$types';
import { api } from '$lib/api';

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
