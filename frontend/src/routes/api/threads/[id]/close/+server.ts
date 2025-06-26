import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ params, fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/threads/${params.id}/close`, {
			method: 'POST'
		});

		if (!response.ok) {
			throw new Error('Failed to close thread');
		}

		const thread = await response.json();
		return json(thread);
	} catch (error) {
		console.error('Error closing thread:', error);
		return json({ error: 'Failed to close thread' }, { status: 500 });
	}
};
