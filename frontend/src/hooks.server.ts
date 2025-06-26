import type { Handle } from '@sveltejs/kit';
import { parseJWT } from '$lib/auth';
import { parse } from 'cookie';

export const handle: Handle = async ({ event, resolve }) => {
	const cookies = parse(event.request.headers.get('cookie') || '');
	const authToken = cookies.auth_token;

	if (authToken) {
		const payload = parseJWT(authToken);
		if (payload) {
			event.locals.user = payload;
		}
	}

	return resolve(event);
}; 