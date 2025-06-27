
import type { PageServerLoad } from './$types';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ locals: { user } }) => {
	try {
		const macros = await api.getAllMacros();
		return {
			macros
		};
	} catch (error) {
		console.error('Error loading macros:', error);
		return {
			macros: [],
			error: 'Failed to load macros'
		};
	}
};
