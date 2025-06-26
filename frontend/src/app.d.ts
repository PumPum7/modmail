// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		interface Error {}
		interface Locals {
			user?: {
				id: string;
				username: string;
				discriminator: string;
				avatar: string | null;
				email: string;
				roles: string[];
				isModerator: boolean;
			};
		}
		interface PageData {}
		interface PageState {}
		interface Platform {}
	}
}

export {};
