import { redirect } from '@sveltejs/kit';
import { exchangeCodeForTokens, getDiscordUser, createJWT } from '$lib/auth';
import type { RequestHandler } from './$types';
import { serialize } from 'cookie';

export const GET: RequestHandler = async ({ url, cookies }) => {
	const code = url.searchParams.get('code');
	const error = url.searchParams.get('error');

	if (error || !code) {
		throw redirect(302, '/login?error=auth_failed');
	}

	try {
		// Exchange code for tokens
		const tokens = await exchangeCodeForTokens(code);

		// Get user info
		const discordUser = await getDiscordUser(tokens.access_token);

		// Create JWT with user info and access token for guild fetching
		const userPayload = {
			id: discordUser.id,
			username: discordUser.username,
			discriminator: discordUser.discriminator,
			avatar: discordUser.avatar,
			email: discordUser.email,
			access_token: tokens.access_token
		};

		const jwt = createJWT(userPayload);

		// Set auth cookie
		const cookie = serialize('auth_token', jwt, {
			path: '/',
			httpOnly: true,
			secure: process.env.NODE_ENV === 'production',
			sameSite: 'lax',
			maxAge: 60 * 60 * 24 * 7 // 1 week
		});

		const response = new Response(null, {
			status: 302,
			headers: {
				Location: '/select-server',
				'Set-Cookie': cookie
			}
		});

		return response;
	} catch (error) {
		console.error('Auth callback error:', error);
		throw redirect(302, '/login?error=auth_failed');
	}
};
