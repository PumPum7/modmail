import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/blocked-users`);
		if (!response.ok) {
			throw new Error('Failed to fetch blocked users');
		}
		const blockedUsers = await response.json();
		return json(blockedUsers);
	} catch (error) {
		console.error('Error fetching blocked users:', error);
		return json({ error: 'Failed to fetch blocked users' }, { status: 500 });
	}
};

export const POST: RequestHandler = async ({ request, fetch }) => {
	try {
		const body = await request.json();
		const response = await fetch(`${PUBLIC_BACKEND_URL}/blocked-users`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		});

		if (!response.ok) {
			throw new Error('Failed to block user');
		}

		const blockedUser = await response.json();
		return json(blockedUser);
	} catch (error) {
		console.error('Error blocking user:', error);
		return json({ error: 'Failed to block user' }, { status: 500 });
	}
};
