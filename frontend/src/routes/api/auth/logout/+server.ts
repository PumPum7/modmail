import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { serialize } from 'cookie';

export const POST: RequestHandler = async () => {
	const cookie = serialize('auth_token', '', {
		path: '/',
		httpOnly: true,
		secure: false, // Set to true in production
		sameSite: 'lax',
		expires: new Date(0) // Expire immediately
	});

	return new Response(null, {
		status: 200,
		headers: {
			'Set-Cookie': cookie
		}
	});
};
