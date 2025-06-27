import type { Thread, MessageData, Macro, Attachment } from "./types.js";

const BACKEND_URL = process.env.PUBLIC_BACKEND_URL || "http://localhost:8080";

export async function createThread(
  userId: string,
  channelId: string
): Promise<Thread> {
  const response = await fetch(`${BACKEND_URL}/threads`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      user_id: userId,
      thread_id: channelId,
    }),
  });
  return response.json() as Promise<Thread>;
}

export async function getThreadByUserId(userId: string): Promise<Thread | null> {
  const response = await fetch(`${BACKEND_URL}/threads`);
  const {threads} = (await response.json()) as {threads: Thread[]};
  return threads.find((t) => t.user_id === userId && t.is_open) || null;
}

export async function getThreadByChannelId(channelId: string): Promise<Thread | null> {
  const response = await fetch(`${BACKEND_URL}/threads`);
  const {threads} = (await response.json()) as {threads: Thread[]};
  return threads.find((t) => t.thread_id === channelId) || null;
}

export async function closeThread(threadId: number): Promise<Thread> {
  const response = await fetch(`${BACKEND_URL}/threads/${threadId}/close`, {
    method: "POST",
  });
  return response.json() as Promise<Thread>;
}

export async function addMessageToThread(
  threadId: number,
  authorId: string,
  authorTag: string,
  content: string,
  attachments: Attachment[] = []
): Promise<MessageData> {
  const response = await fetch(`${BACKEND_URL}/threads/${threadId}/messages`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      author_id: authorId,
      author_tag: authorTag,
      content: content,
      attachments: attachments,
    }),
  });
  return response.json() as Promise<MessageData>;
}

export async function addNoteToThread(
  threadId: number,
  authorId: string,
  authorTag: string,
  content: string
): Promise<any> {
  const response = await fetch(`${BACKEND_URL}/threads/${threadId}/notes`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      author_id: authorId,
      author_tag: authorTag,
      content: content,
    }),
  });
  return response.json();
}

export async function createMacro(name: string, content: string): Promise<Macro> {
  const response = await fetch(`${BACKEND_URL}/macros`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ name, content }),
  });
  return response.json() as Promise<Macro>;
}

export async function getMacroByName(name: string): Promise<Macro | null> {
  const response = await fetch(
    `${BACKEND_URL}/macros/${encodeURIComponent(name)}`
  );
  const result = await response.json();
  return result === null ? null : (result as Macro);
}

export async function deleteMacro(name: string): Promise<{ success: boolean; message: string }> {
  const response = await fetch(
    `${BACKEND_URL}/macros/${encodeURIComponent(name)}`,
    {
      method: "DELETE",
    }
  );
  return response.json() as Promise<{ success: boolean; message: string }>;
}

export async function getMacros(): Promise<Macro[]> {
  const response = await fetch(`${BACKEND_URL}/macros`);
  return response.json() as Promise<Macro[]>;
}

export async function editMacro(name: string, content: string): Promise<Macro> {
  const response = await fetch(
    `${BACKEND_URL}/macros/${encodeURIComponent(name)}`,
    {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ content }),
    }
  );
  return response.json() as Promise<Macro>;
}

export async function blockUser(
  userId: string,
  userTag: string,
  blockedBy: string,
  blockedByTag: string,
  reason?: string
): Promise<any> {
  const response = await fetch(`${BACKEND_URL}/blocked-users`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      user_id: userId,
      user_tag: userTag,
      blocked_by: blockedBy,
      blocked_by_tag: blockedByTag,
      reason: reason || null,
    }),
  });
  return response.json();
}

export async function isUserBlocked(userId: string): Promise<boolean> {
  const response = await fetch(`${BACKEND_URL}/blocked-users/${userId}`);
  const result = (await response.json()) as { blocked: boolean };
  return result.blocked;
} 

export async function unblockUser(userId: string): Promise<any> {
  const response = await fetch(`${BACKEND_URL}/blocked-users/${userId}`, {
    method: "DELETE",
  });
  return response.json();
}