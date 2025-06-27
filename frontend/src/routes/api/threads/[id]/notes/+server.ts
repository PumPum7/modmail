import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ params, request, fetch }) => {
	try {
		const body = await request.json();
		const response = await fetch(`${PUBLIC_BACKEND_URL}/threads/${params.id}/notes`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		});

		if (!response.ok) {
			throw new Error('Failed to add note to thread');
		}

		const note = await response.json();
		return json(note);
	} catch (error) {
		console.error('Error adding note to thread:', error);
		return json({ error: 'Failed to add note to thread' }, { status: 500 });
	}
};
