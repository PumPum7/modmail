import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/threads`);
		if (!response.ok) {
			throw new Error('Failed to fetch threads');
		}
		const threads = await response.json();
		return json(threads);
	} catch (error) {
		console.error('Error fetching threads:', error);
		return json({ error: 'Failed to fetch threads' }, { status: 500 });
	}
};
