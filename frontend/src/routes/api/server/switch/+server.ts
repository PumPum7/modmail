import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const data = await request.formData();
	const guildId = data.get('guild_id') as string;

	if (!guildId) {
		return new Response('Guild ID required', { status: 400 });
	}

	// Set selected guild cookie
	cookies.set('selected_guild_id', guildId, {
		path: '/',
		httpOnly: true,
		secure: process.env.NODE_ENV === 'production',
		sameSite: 'lax',
		maxAge: 60 * 60 * 24 * 30 // 30 days
	});

	throw redirect(302, '/');
};
