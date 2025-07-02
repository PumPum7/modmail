import type { PageServerLoad, Actions } from './$types';
import type { BlockedUser } from '$lib/api';
import { api } from '$lib/api';
import { fail, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ locals, cookies }) => {
	const user = locals.user;
	const selectedGuildId = cookies.get('selected_guild_id');

	if (!user) {
		throw redirect(302, '/login');
	}

	if (!selectedGuildId) {
		throw redirect(302, '/select-server');
	}

	try {
		const blockedUsers: BlockedUser[] = await api.getAllBlockedUsers(selectedGuildId);
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
	block: async ({ request, locals, cookies }) => {
		const user = locals.user;
		const selectedGuildId = cookies.get('selected_guild_id');

		if (!user) {
			return fail(401, {
				error: 'Authentication required'
			});
		}

		if (!selectedGuildId) {
			return fail(400, { error: 'No server selected' });
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
			await api.blockUser(selectedGuildId, {
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

	unblock: async ({ request, locals, cookies }) => {
		const user = locals.user;
		const selectedGuildId = cookies.get('selected_guild_id');

		if (!user) {
			return fail(401, {
				error: 'Authentication required'
			});
		}

		if (!selectedGuildId) {
			return fail(400, { error: 'No server selected' });
		}

		const data = await request.formData();
		const userId = data.get('user_id')?.toString();
		const userTag = data.get('user_tag')?.toString();

		if (!userId) {
			return fail(400, {
				error: 'User ID is required'
			});
		}

		try {
			await api.unblockUser(selectedGuildId, userId);

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
