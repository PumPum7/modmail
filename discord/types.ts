export interface Thread {
	id: number;
	user_id: string;
	thread_id: string;
	is_open: boolean;
}

export interface MessageData {
	id: string;
	author_id: string;
	author_tag: string;
	content: string;
	attachments: any[];
	created_at: string;
}

export interface Macro {
	id: number;
	name: string;
	content: string;
}

export interface Attachment {
	url: string;
	filename: string;
	content_type: string;
	size: number;
}

export interface BlockUserResponse {
	success: boolean;
	message: string;
}
