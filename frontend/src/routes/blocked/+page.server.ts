import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { PageServerLoad } from './$types';
import type { BlockedUser } from '$lib/api';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ locals: { user } }) => {
	try {
		const blockedUsers: BlockedUser[] = await api.getAllBlockedUsers();
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
