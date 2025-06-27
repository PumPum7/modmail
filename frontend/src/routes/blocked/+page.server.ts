import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { PageServerLoad, Actions } from './$types';
import type { BlockedUser } from '$lib/api';
import { api } from '$lib/api';
import { fail } from '@sveltejs/kit';

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

export const actions: Actions = {
	block: async ({ request, locals: { user } }) => {
		if (!user) {
			return fail(401, {
				error: 'Authentication required'
			});
		}

		const data = await request.formData();
		const user_id = data.get('user_id')?.toString();
		const user_tag = data.get('user_tag')?.toString();
		const reason = data.get('reason')?.toString();

		if (!user_id?.trim() || !user_tag?.trim()) {
			return fail(400, {
				error: 'User ID and User Tag are required'
			});
		}

		try {
			await api.blockUser({
				user_id: user_id.trim(),
				user_tag: user_tag.trim(),
				blocked_by: user.id,
				blocked_by_tag: user.username,
				reason: reason?.trim() || undefined
			});

			return {
				success: 'User blocked successfully!'
			};
		} catch (error) {
			console.error('Error blocking user:', error);
			return fail(500, {
				error: 'Failed to block user'
			});
		}
	},

	unblock: async ({ request, locals: { user } }) => {
		if (!user) {
			return fail(401, {
				error: 'Authentication required'
			});
		}

		const data = await request.formData();
		const userId = data.get('userId')?.toString();
		const userTag = data.get('userTag')?.toString();

		if (!userId) {
			return fail(400, {
				error: 'User ID is required'
			});
		}

		try {
			await api.unblockUser(userId);

			return {
				success: `User ${userTag || ''} unblocked successfully!`
			};
		} catch (error) {
			console.error('Error unblocking user:', error);
			return fail(500, {
				error: 'Failed to unblock user'
			});
		}
	}
};
