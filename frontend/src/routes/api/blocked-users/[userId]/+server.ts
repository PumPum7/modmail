import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const DELETE: RequestHandler = async ({ params, fetch, cookies }) => {
	const selectedGuildId = cookies.get('selected_guild_id');

	if (!selectedGuildId) {
		return json({ error: 'No server selected' }, { status: 400 });
	}

	try {
		const response = await fetch(
			`${PUBLIC_BACKEND_URL}/guilds/${selectedGuildId}/blocked-users/${params.userId}`,
			{
				method: 'DELETE'
			}
		);

		if (!response.ok) {
			throw new Error('Failed to unblock user');
		}

		const result = await response.json();
		return json(result);
	} catch (error) {
		console.error('Error unblocking user:', error);
		return json({ error: 'Failed to unblock user' }, { status: 500 });
	}
};
