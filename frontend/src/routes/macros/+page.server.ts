import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/macros`);
		if (!response.ok) {
			throw new Error('Failed to fetch macros');
		}
		const macros = await response.json();
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
