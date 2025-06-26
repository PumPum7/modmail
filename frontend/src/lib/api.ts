import { PUBLIC_BACKEND_URL } from '$env/static/public';

export interface Message {
	id: string;
	author_id: string;
	author_tag: string;
	content: string;
	created_at: number;
	thread_id: number;
}

export interface Thread {
	id: number;
	user_id: string;
	thread_id: string;
	is_open: boolean;
}

export interface Macro {
	id: number;
	name: string;
	content: string;
}

export interface ThreadWithMessages {
	thread: Thread;
	messages: Message[];
}

export class ApiClient {
	private baseUrl: string;

	constructor() {
		this.baseUrl = PUBLIC_BACKEND_URL;
	}

	async getAllMessages(): Promise<Message[]> {
		const response = await fetch(`${this.baseUrl}/messages`);
		if (!response.ok) {
			throw new Error('Failed to fetch messages');
		}
		return response.json();
	}

	async getAllThreads(): Promise<Thread[]> {
		const response = await fetch(`${this.baseUrl}/threads`);
		if (!response.ok) {
			throw new Error('Failed to fetch threads');
		}
		return response.json();
	}

	async getThread(id: number): Promise<ThreadWithMessages> {
		const response = await fetch(`${this.baseUrl}/threads/${id}`);
		if (!response.ok) {
			throw new Error('Failed to fetch thread');
		}
		const [thread, messages] = await response.json();
		return { thread, messages };
	}

	async closeThread(id: number): Promise<Thread> {
		const response = await fetch(`${this.baseUrl}/threads/${id}/close`, {
			method: 'POST'
		});
		if (!response.ok) {
			throw new Error('Failed to close thread');
		}
		return response.json();
	}

	async addMessageToThread(
		threadId: number,
		message: {
			author_id: string;
			author_tag: string;
			content: string;
		}
	): Promise<Message> {
		const response = await fetch(`${this.baseUrl}/threads/${threadId}/messages`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(message)
		});
		if (!response.ok) {
			throw new Error('Failed to add message to thread');
		}
		return response.json();
	}

	async getAllMacros(): Promise<Macro[]> {
		const response = await fetch(`${this.baseUrl}/macros`);
		if (!response.ok) {
			throw new Error('Failed to fetch macros');
		}
		return response.json();
	}

	async getMacro(name: string): Promise<Macro | null> {
		const response = await fetch(`${this.baseUrl}/macros/${encodeURIComponent(name)}`);
		if (!response.ok) {
			return null;
		}
		const result = await response.json();
		return result === null ? null : result;
	}

	async createMacro(macro: { name: string; content: string }): Promise<Macro> {
		const response = await fetch(`${this.baseUrl}/macros`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(macro)
		});
		if (!response.ok) {
			throw new Error('Failed to create macro');
		}
		return response.json();
	}

	async deleteMacro(name: string): Promise<{ success: boolean; message: string }> {
		const response = await fetch(`${this.baseUrl}/macros/${encodeURIComponent(name)}`, {
			method: 'DELETE'
		});
		if (!response.ok) {
			throw new Error('Failed to delete macro');
		}
		return response.json();
	}

	async updateMacro(name: string, content: string): Promise<Macro> {
		const response = await fetch(`${this.baseUrl}/macros/${encodeURIComponent(name)}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ name, content })
		});
		if (!response.ok) {
			throw new Error('Failed to update macro');
		}
		return response.json();
	}
}

export const api = new ApiClient();
