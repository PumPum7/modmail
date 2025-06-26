import {
	PUBLIC_DISCORD_CLIENT_ID,
	PUBLIC_DISCORD_REDIRECT_URI,
	PUBLIC_MOD_ROLE_IDS,
	PUBLIC_DISCORD_SERVER_ID
} from '$env/static/public';
import { DISCORD_CLIENT_SECRET } from '$env/static/private';

export interface DiscordUser {
	id: string;
	username: string;
	discriminator: string;
	avatar: string | null;
	email: string;
}

export interface DiscordGuildMember {
	user: DiscordUser;
	nick: string | null;
	roles: string[];
}

export function getDiscordAuthUrl(): string {
	const params = new URLSearchParams({
		client_id: PUBLIC_DISCORD_CLIENT_ID,
		redirect_uri: PUBLIC_DISCORD_REDIRECT_URI,
		response_type: 'code',
		scope: 'identify email guilds.members.read'
	});

	return `https://discord.com/oauth2/authorize?${params.toString()}`;
}

export async function exchangeCodeForTokens(code: string): Promise<{
	access_token: string;
	refresh_token: string;
	expires_in: number;
}> {
	const response = await fetch('https://discord.com/api/oauth2/token', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/x-www-form-urlencoded'
		},
		body: new URLSearchParams({
			client_id: PUBLIC_DISCORD_CLIENT_ID,
			client_secret: DISCORD_CLIENT_SECRET,
			grant_type: 'authorization_code',
			code: code,
			redirect_uri: PUBLIC_DISCORD_REDIRECT_URI
		})
	});

	if (!response.ok) {
		throw new Error('Failed to exchange code for tokens');
	}

	return response.json();
}

export async function getDiscordUser(accessToken: string): Promise<DiscordUser> {
	const response = await fetch('https://discord.com/api/users/@me', {
		headers: {
			Authorization: `Bearer ${accessToken}`
		}
	});

	if (!response.ok) {
		throw new Error('Failed to get Discord user');
	}

	return response.json();
}

export async function getGuildMember(
	accessToken: string,
	_userId: string
): Promise<DiscordGuildMember | null> {
	try {
		const response = await fetch(
			`https://discord.com/api/users/@me/guilds/${PUBLIC_DISCORD_SERVER_ID}/member`,
			{
				headers: {
					Authorization: `Bearer ${accessToken}`
				}
			}
		);

		if (!response.ok) {
			return null;
		}

		return response.json();
	} catch {
		return null;
	}
}

export function isModerator(roles: string[]): boolean {
	const modRoleIds = PUBLIC_MOD_ROLE_IDS.split(',').map((id) => id.trim());
	return roles.some((role) => modRoleIds.includes(role));
}

export function createJWT(payload: Record<string, any>): string {
	// Simple JWT implementation for demo purposes
	// In production, use a proper JWT library
	const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
	const body = btoa(JSON.stringify(payload));
	const signature = btoa('signature'); // In production, use proper signing
	return `${header}.${body}.${signature}`;
}

export function parseJWT(token: string): any {
	try {
		const parts = token.split('.');
		if (parts.length !== 3) return null;
		return JSON.parse(atob(parts[1]));
	} catch {
		return null;
	}
}
