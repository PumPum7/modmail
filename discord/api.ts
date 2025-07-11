import type { Thread, MessageData, Macro, Attachment } from './types.js';

const BACKEND_URL = process.env.PUBLIC_BACKEND_URL || 'http://localhost:8080';

// Guild Configuration API
export async function getGuildConfig(guildId: string): Promise<any> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/config`);
	if (!response.ok) {
		throw new Error('Guild config not found');
	}
	return response.json();
}

export async function createGuildConfig(guildId: string, config: any): Promise<any> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/config`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(config),
	});
	return response.json();
}

export async function updateGuildConfig(guildId: string, config: any): Promise<any> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/config`, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(config),
	});
	return response.json();
}

// Updated API calls with guild_id
export async function createThread(
	userId: string,
	channelId: string,
	guildId: string,
	urgency?: string
): Promise<Thread> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/threads`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			user_id: userId,
			thread_id: channelId,
			guild_id: guildId,
			urgency: urgency || 'Medium',
		}),
	});
	return response.json() as Promise<Thread>;
}

export async function getThreadByUserId(userId: string, guildId: string): Promise<Thread | null> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/threads`);
	const { threads } = (await response.json()) as { threads: Thread[] };
	return threads.find((t) => t.user_id === userId && t.is_open) || null;
}

export async function getThreadByChannelId(
	channelId: string,
	guildId: string
): Promise<Thread | null> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/threads`);
	const { threads } = (await response.json()) as { threads: Thread[] };
	return threads.find((t) => t.thread_id === channelId) || null;
}

export async function closeThread(
	threadId: number,
	guildId: string,
	closedBy: { id: string; tag: string }
): Promise<Thread> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/threads/${threadId}/close`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			closed_by_id: closedBy.id,
			closed_by_tag: closedBy.tag,
		}),
	});
	return response.json() as Promise<Thread>;
}

export async function addMessageToThread(
	threadId: number,
	guildId: string,
	authorId: string,
	authorTag: string,
	content: string,
	attachments: Attachment[] = []
): Promise<MessageData> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/threads/${threadId}/messages`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			author_id: authorId,
			author_tag: authorTag,
			content: content,
			attachments: attachments,
			guild_id: guildId,
		}),
	});
	return response.json() as Promise<MessageData>;
}

export async function addNoteToThread(
	threadId: number,
	guildId: string,
	authorId: string,
	authorTag: string,
	content: string
): Promise<any> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/threads/${threadId}/notes`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			author_id: authorId,
			author_tag: authorTag,
			content: content,
			guild_id: guildId,
		}),
	});
	return response.json();
}

export async function createMacro(name: string, content: string, guildId: string): Promise<Macro> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/macros`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ name, content, guild_id: guildId }),
	});
	return response.json() as Promise<Macro>;
}

export async function getMacroByName(name: string, guildId: string): Promise<Macro | null> {
	const response = await fetch(
		`${BACKEND_URL}/guilds/${guildId}/macros/${encodeURIComponent(name)}`
	);
	const result = await response.json();
	return result === null ? null : (result as Macro);
}

export async function deleteMacro(
	name: string,
	guildId: string
): Promise<{ success: boolean; message: string }> {
	const response = await fetch(
		`${BACKEND_URL}/guilds/${guildId}/macros/${encodeURIComponent(name)}`,
		{
			method: 'DELETE',
		}
	);
	return response.json() as Promise<{ success: boolean; message: string }>;
}

export async function getMacros(guildId: string): Promise<Macro[]> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/macros`);
	return response.json() as Promise<Macro[]>;
}

export async function editMacro(name: string, content: string, guildId: string): Promise<Macro> {
	const response = await fetch(
		`${BACKEND_URL}/guilds/${guildId}/macros/${encodeURIComponent(name)}`,
		{
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ content, guild_id: guildId }),
		}
	);
	return response.json() as Promise<Macro>;
}

export async function blockUser(
	userId: string,
	userTag: string,
	blockedBy: string,
	blockedByTag: string,
	guildId: string,
	reason?: string
): Promise<any> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/blocked-users`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			user_id: userId,
			user_tag: userTag,
			blocked_by: blockedBy,
			blocked_by_tag: blockedByTag,
			reason: reason || null,
			guild_id: guildId,
		}),
	});
	return response.json();
}

export async function isUserBlocked(userId: string, guildId: string): Promise<boolean> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/blocked-users/${userId}`);
	const result = (await response.json()) as { blocked: boolean };
	return result.blocked;
}

export async function unblockUser(userId: string, guildId: string): Promise<any> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/blocked-users/${userId}`, {
		method: 'DELETE',
	});
	return response.json();
}

export async function updateThreadUrgency(
	threadId: number,
	guildId: string,
	urgency: string
): Promise<Thread> {
	const response = await fetch(`${BACKEND_URL}/guilds/${guildId}/threads/${threadId}/urgency`, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ urgency }),
	});
	return response.json() as Promise<Thread>;
}

// Get servers shared between user and bot (from bot's perspective)
export async function getUserServers(userId: string, client: any): Promise<any[]> {
	const sharedServers = [];

	// Get all guilds the bot is in
	for (const [guildId, guild] of client.guilds.cache) {
		try {
			// Check if the user is a member of this guild
			const member = await guild.members.fetch(userId).catch(() => null);

			if (member) {
				// User is in this guild, add to shared servers
				sharedServers.push({
					guild_id: guildId,
					guild_name: guild.name,
					guild_icon: guild.icon,
					member_count: guild.memberCount,
					user_has_permissions: true, // We know they're a member
				});
			}
		} catch (error) {
			// If we can't fetch the member, skip this guild
			console.log(`Could not check membership for user ${userId} in guild ${guild.name}`);
			continue;
		}
	}

	// Validate with backend to ensure only configured servers are returned
	if (sharedServers.length === 0) {
		return [];
	}

	try {
		const response = await fetch(`${BACKEND_URL}/validate-guilds`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(sharedServers),
		});

		if (response.ok) {
			const validatedServers = (await response.json()) as any[];
			return validatedServers.filter((server: any) => server.has_bot && server.has_config);
		}
	} catch (error) {
		console.error('Failed to validate servers with backend:', error);
	}

	// Fallback: return all shared servers if backend validation fails
	return sharedServers;
}

export async function createServer(guildId: string, guildName: string): Promise<any> {
	const response = await fetch(`${BACKEND_URL}/servers`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			guild_id: guildId,
			guild_name: guildName,
		}),
	});

	if (!response.ok) {
		throw new Error(`Failed to create server: ${response.statusText}`);
	}

	return response.json();
}
