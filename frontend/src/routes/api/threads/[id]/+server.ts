import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ params, fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/threads/${params.id}`);
		if (!response.ok) {
			throw new Error('Failed to fetch thread');
		}
		const [thread, messages] = await response.json();
		return json({ thread, messages });
	} catch (error) {
		console.error('Error fetching thread:', error);
		return json({ error: 'Failed to fetch thread' }, { status: 500 });
	}
}; 