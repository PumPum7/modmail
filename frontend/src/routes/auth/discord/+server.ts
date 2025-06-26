import { redirect } from '@sveltejs/kit';
import { getDiscordAuthUrl } from '$lib/auth';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
	const authUrl = getDiscordAuthUrl();
	throw redirect(302, authUrl);
}; 