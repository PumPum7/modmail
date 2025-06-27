import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ params, fetch, locals }) => {
	try {
		const body = locals.user ? {
			closed_by_id: locals.user.id,
			closed_by_tag: locals.user.username
		} : undefined;

		const response = await fetch(`${PUBLIC_BACKEND_URL}/threads/${params.id}/close`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: body ? JSON.stringify(body) : undefined
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
