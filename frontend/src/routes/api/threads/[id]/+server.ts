import { json } from '@sveltejs/kit';
import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ params, fetch, cookies, url }) => {
	const selectedGuildId = cookies.get('selected_guild_id');

	if (!selectedGuildId) {
		return json({ error: 'No server selected' }, { status: 400 });
	}

	try {
		const page = url.searchParams.get('page') || '1';
		const limit = url.searchParams.get('limit') || '50';

		const response = await fetch(
			`${PUBLIC_BACKEND_URL}/guilds/${selectedGuildId}/threads/${params.id}?page=${page}&limit=${limit}`
		);
		if (!response.ok) {
			throw new Error('Failed to fetch thread');
		}
		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('Error fetching thread:', error);
		return json({ error: 'Failed to fetch thread' }, { status: 500 });
	}
};
