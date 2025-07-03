import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ cookies }) => {
	cookies.delete('auth_token', { path: '/' });
	cookies.delete('selected_guild_id', { path: '/' });

	redirect(302, '/login');
};
