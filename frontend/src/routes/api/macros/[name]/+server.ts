import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const PUT: RequestHandler = async ({ params, request, fetch }) => {
	try {
		const body = await request.json();
		const response = await fetch(`${PUBLIC_BACKEND_URL}/macros/${encodeURIComponent(params.name as string)}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		});
		
		if (!response.ok) {
			throw new Error('Failed to update macro');
		}
		
		const macro = await response.json();
		return json(macro);
	} catch (error) {
		console.error('Error updating macro:', error);
		return json({ error: 'Failed to update macro' }, { status: 500 });
	}
};

export const DELETE: RequestHandler = async ({ params, fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/macros/${encodeURIComponent(params.name as string)}`, {
			method: 'DELETE'
		});
		
		if (!response.ok) {
			throw new Error('Failed to delete macro');
		}
		
		const result = await response.json();
		return json(result);
	} catch (error) {
		console.error('Error deleting macro:', error);
		return json({ error: 'Failed to delete macro' }, { status: 500 });
	}
}; 