import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/macros`);
		if (!response.ok) {
			throw new Error('Failed to fetch macros');
		}
		const macros = await response.json();
		return json(macros);
	} catch (error) {
		console.error('Error fetching macros:', error);
		return json({ error: 'Failed to fetch macros' }, { status: 500 });
	}
};

export const POST: RequestHandler = async ({ request, fetch }) => {
	try {
		const body = await request.json();
		const response = await fetch(`${PUBLIC_BACKEND_URL}/macros`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		});

		if (!response.ok) {
			throw new Error('Failed to create macro');
		}

		const macro = await response.json();
		return json(macro);
	} catch (error) {
		console.error('Error creating macro:', error);
		return json({ error: 'Failed to create macro' }, { status: 500 });
	}
};
