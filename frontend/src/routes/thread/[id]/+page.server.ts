import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { PageServerLoad } from './$types';
import type { Note } from '$lib/api';

export const load: PageServerLoad = async ({ params, fetch, locals: { user }, url }) => {
	try {
		const page = parseInt(url.searchParams.get('page') || '1');
		const limit = parseInt(url.searchParams.get('limit') || '50');

		const response = await fetch(
			`${PUBLIC_BACKEND_URL}/threads/${params.id}?page=${page}&limit=${limit}`
		);
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
		const data = await response.json();

		const notesResponse = await fetch(`${PUBLIC_BACKEND_URL}/threads/${params.id}/notes`);
		let notes: Note[] = [];
		if (notesResponse.ok) {
			notes = await notesResponse.json();
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
