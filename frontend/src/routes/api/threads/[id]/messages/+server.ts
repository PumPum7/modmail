import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ params, request, fetch }) => {
	try {
		const body = await request.json();
		const response = await fetch(`${PUBLIC_BACKEND_URL}/threads/${params.id}/messages`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		});
		
		if (!response.ok) {
			throw new Error('Failed to add message to thread');
		}
		
		const message = await response.json();
		return json(message);
	} catch (error) {
		console.error('Error adding message to thread:', error);
		return json({ error: 'Failed to add message to thread' }, { status: 500 });
	}
}; 