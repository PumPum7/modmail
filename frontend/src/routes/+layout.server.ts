import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals, cookies, url }) => {
	const user = locals.user;

	// If user is authenticated but no guild is selected, redirect to server selection
	if (
		user &&
		!cookies.get('selected_guild_id') &&
		url.pathname !== '/select-server' &&
		url.pathname !== '/login'
	) {
		throw redirect(302, '/select-server');
	}

	// If user is not authenticated and trying to access protected routes, redirect to login
	if (!user && url.pathname !== '/login' && url.pathname !== '/select-server') {
		throw redirect(302, '/login');
	}

	return {
		user: user || null,
		selectedGuildId: cookies.get('selected_guild_id') || null
	};
};
