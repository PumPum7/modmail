import { PUBLIC_BACKEND_URL } from '$env/static/public';

export interface Attachment {
	url: string;
	filename: string;
	content_type: string;
	size: number;
}

export interface Message {
	id: string;
	author_id: string;
	author_tag: string;
	content: string;
	attachments: Attachment[];
	created_at: number;
	thread_id: number;
}

export interface Thread {
	id: number;
	user_id: string;
	thread_id: string;
	is_open: boolean;
	urgency: string;
}

export interface Macro {
	id: number;
	name: string;
	content: string;
	quick_access: boolean;
}

export interface Note {
	id: string;
	thread_id: number;
	author_id: string;
	author_tag: string;
	content: string;
	created_at: number;
}

export interface BlockedUser {
	id: number;
	user_id: string;
	user_tag: string;
	blocked_by: string;
	blocked_by_tag: string;
	reason: string | null;
	created_at: number;
}

export interface ThreadWithMessages {
	thread: Thread;
	messages: Message[];
	pagination?: PaginationInfo;
}

export interface PaginationInfo {
	page: number;
	limit: number;
	total_count: number;
	total_pages: number;
	has_next: boolean;
	has_prev: boolean;
}

export interface ThreadsResponse {
	threads: Thread[];
	pagination: PaginationInfo;
}

export interface AnalyticsOverview {
	total_threads: number;
	open_threads: number;
	closed_threads: number;
	total_messages: number;
	total_notes: number;
	blocked_users: number;
	avg_response_time_hours: number | null;
	threads_today: number;
	threads_this_week: number;
	threads_this_month: number;
}

export interface ThreadVolumeData {
	date: string;
	count: number;
}

export interface ModeratorActivity {
	moderator_tag: string;
	message_count: number;
	note_count: number;
	threads_closed: number;
}

export interface ResponseTimeMetrics {
	avg_first_response_hours: number | null;
	avg_resolution_time_hours: number | null;
	median_first_response_hours: number | null;
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

	async getAllThreads(page: number = 1, limit: number = 20): Promise<ThreadsResponse> {
		const response = await fetch(`${this.baseUrl}/threads?page=${page}&limit=${limit}`);
		if (!response.ok) {
			throw new Error('Failed to fetch threads');
		}
		return response.json();
	}

	async getThread(id: string, page: number = 1, limit: number = 50): Promise<ThreadWithMessages> {
		const response = await fetch(`${this.baseUrl}/threads/${id}?page=${page}&limit=${limit}`);
		if (!response.ok) {
			throw new Error('Failed to fetch thread');
		}
		const data = await response.json();
		return {
			thread: data.thread,
			messages: data.messages,
			pagination: data.pagination
		};
	}

	async getThreadNotes(id: string): Promise<Note[]> {
		const response = await fetch(`${this.baseUrl}/threads/${id}/notes`);
		if (!response.ok) {
			throw new Error('Failed to fetch thread notes');
		}
		return response.json();
	}

	async addNoteToThread(
		threadId: number,
		note: {
			author_id: string;
			author_tag: string;
			content: string;
		}
	): Promise<Note> {
		const response = await fetch(`${this.baseUrl}/threads/${threadId}/notes`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(note)
		});
		if (!response.ok) {
			throw new Error('Failed to add note to thread');
		}
		return response.json();
	}

	async closeThread(id: number, closedBy?: { id: string; tag: string }): Promise<Thread> {
		const response = await fetch(`${this.baseUrl}/threads/${id}/close`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: closedBy
				? JSON.stringify({
						closed_by_id: closedBy.id,
						closed_by_tag: closedBy.tag
					})
				: undefined
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
			attachments?: Attachment[];
		}
	): Promise<Message> {
		const response = await fetch(`${this.baseUrl}/threads/${threadId}/messages`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				...message,
				attachments: message.attachments || []
			})
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

	async getAllBlockedUsers(): Promise<BlockedUser[]> {
		const response = await fetch(`${this.baseUrl}/blocked-users`);
		if (!response.ok) {
			throw new Error('Failed to fetch blocked users');
		}
		return response.json();
	}

	async blockUser(blockedUser: {
		user_id: string;
		user_tag: string;
		blocked_by: string;
		blocked_by_tag: string;
		reason?: string;
	}): Promise<BlockedUser> {
		const response = await fetch(`${this.baseUrl}/blocked-users`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(blockedUser)
		});
		if (!response.ok) {
			throw new Error('Failed to block user');
		}
		return response.json();
	}

	async unblockUser(userId: string): Promise<{ success: boolean; message: string }> {
		const response = await fetch(`${this.baseUrl}/blocked-users/${encodeURIComponent(userId)}`, {
			method: 'DELETE'
		});
		if (!response.ok) {
			throw new Error('Failed to unblock user');
		}
		return response.json();
	}

	async isUserBlocked(userId: string): Promise<{ blocked: boolean; user?: BlockedUser }> {
		const response = await fetch(`${this.baseUrl}/blocked-users/${encodeURIComponent(userId)}`);
		if (!response.ok) {
			throw new Error('Failed to check if user is blocked');
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

	async createMacro(macro: {
		name: string;
		content: string;
		quick_access?: boolean;
	}): Promise<Macro> {
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

	async updateMacro(name: string, content: string, quick_access?: boolean): Promise<Macro> {
		const response = await fetch(`${this.baseUrl}/macros/${encodeURIComponent(name)}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ name, content, quick_access })
		});
		if (!response.ok) {
			throw new Error('Failed to update macro');
		}
		return response.json();
	}

	async updateThreadUrgency(threadId: number, urgency: string): Promise<Thread> {
		const response = await fetch(`${this.baseUrl}/threads/${threadId}/urgency`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ urgency })
		});
		if (!response.ok) {
			throw new Error('Failed to update thread urgency');
		}
		return response.json();
	}

	async getAnalyticsOverview(): Promise<AnalyticsOverview> {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/analytics/overview`);
		if (!response.ok) throw new Error('Failed to fetch analytics overview');
		return response.json();
	}

	async getThreadVolume(): Promise<ThreadVolumeData[]> {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/analytics/thread-volume`);
		if (!response.ok) throw new Error('Failed to fetch thread volume data');
		return response.json();
	}

	async getModeratorActivity(): Promise<ModeratorActivity[]> {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/analytics/moderator-activity`);
		if (!response.ok) throw new Error('Failed to fetch moderator activity');
		return response.json();
	}

	async getResponseTimes(): Promise<ResponseTimeMetrics> {
		const response = await fetch(`${PUBLIC_BACKEND_URL}/analytics/response-times`);
		if (!response.ok) throw new Error('Failed to fetch response times');
		return response.json();
	}
}

export const api = new ApiClient();
