import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { PageServerLoad } from './$types';
import type { BlockedUser } from '$lib/api';

export const load: PageServerLoad = async ({ fetch, locals: { user } }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/blocked-users`);
		if (!response.ok) {
			throw new Error('Failed to fetch blocked users');
		}
		const blockedUsers: BlockedUser[] = await response.json();
		return {
			blockedUsers,
			user
		};
	} catch (error) {
		console.error('Error loading blocked users:', error);
		return {
			blockedUsers: [],
			user,
			error: 'Failed to load blocked users'
		};
	}
};
