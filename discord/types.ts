export interface Thread {
	id: number;
	user_id: string;
	thread_id: string;
	is_open: boolean;
	guild_id: string;
}

export interface MessageData {
	id: string;
	author_id: string;
	author_tag: string;
	content: string;
	attachments: any[];
	created_at: string;
	guild_id: string;
}

export interface Macro {
	id: number;
	name: string;
	content: string;
	guild_id: string;
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

export interface GuildConfig {
	id: number;
	guild_id: string;
	modmail_category_id?: string;
	log_channel_id?: string;
	randomize_names: boolean;
	auto_close_hours?: number;
	welcome_message?: string;
	moderator_role_ids: string[];
	blocked_words: string[];
	created_at: string;
	updated_at: string;
}

export interface Server {
	id: number;
	guild_id: string;
	guild_name: string;
	is_premium: boolean;
	max_threads: number;
	max_macros: number;
	created_at: string;
	updated_at: string;
}

export interface UserServer {
	id: string;
	name: string;
	icon?: string;
}
