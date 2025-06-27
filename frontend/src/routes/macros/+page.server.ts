import type { PageServerLoad, Actions } from './$types';
import { api } from '$lib/api';
import { fail } from '@sveltejs/kit';

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

export const actions: Actions = {
	create: async ({ request }) => {
		const data = await request.formData();
		const name = data.get('name')?.toString();
		const content = data.get('content')?.toString();
		const quick_access = data.get('quick_access') === 'true';

		if (!name?.trim() || !content?.trim()) {
			return fail(400, {
				error: 'Both name and content are required'
			});
		}

		try {
			await api.createMacro({
				name: name.trim(),
				content: content.trim(),
				quick_access
			});

			return {
				success: 'Macro created successfully!'
			};
		} catch (error) {
			console.error('Error creating macro:', error);
			return fail(500, {
				error: 'Failed to create macro'
			});
		}
	},

	update: async ({ request }) => {
		const data = await request.formData();
		const name = data.get('name')?.toString();
		const content = data.get('content')?.toString();
		const quick_access = data.get('quick_access') === 'true';

		if (!name || !content?.trim()) {
			return fail(400, {
				error: 'Name and content are required'
			});
		}

		try {
			await api.updateMacro(name, content.trim(), quick_access);

			return {
				success: 'Macro updated successfully!'
			};
		} catch (error) {
			console.error('Error updating macro:', error);
			return fail(500, {
				error: 'Failed to update macro'
			});
		}
	},

	delete: async ({ request }) => {
		const data = await request.formData();
		const name = data.get('name')?.toString();

		if (!name) {
			return fail(400, {
				error: 'Macro name is required'
			});
		}

		try {
			const result = await api.deleteMacro(name);

			if (result.success) {
				return {
					success: 'Macro deleted successfully!'
				};
			} else {
				return fail(500, {
					error: result.message || 'Failed to delete macro'
				});
			}
		} catch (error) {
			console.error('Error deleting macro:', error);
			return fail(500, {
				error: 'Failed to delete macro'
			});
		}
	}
};
