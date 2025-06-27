<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { goto } from '$app/navigation';
	import {
		Clock,
		MessageCircle,
		User,
		ArrowLeft,
		XCircle,
		StickyNote,
		ChevronLeft,
		ChevronRight
	} from 'lucide-svelte';
	import type { PageProps } from './$types';
	import { formatDate, formatFileSize } from '$lib/util';

	let { data }: PageProps = $props();

	let loading = $state(false);
	let error = $state('');
	let success = $state('');
	let newNoteContent = $state('');

	// Handle server errors
	$effect.pre(() => {
		if (data.error) {
			error = data.error;
		}
	});

	// Handle authentication
	$effect(() => {
		if (typeof window !== 'undefined') {
			if (!data.user) {
				goto('/login');
				return;
			}

			if (!data.user.isModerator) {
				goto('/login?error=not_moderator');
				return;
			}
		}
	});

	async function addNote() {
		if (!newNoteContent.trim()) {
			error = 'Note content is required';
			return;
		}

		try {
			loading = true;
			error = '';

			if (!data.thread) {
				throw new Error('Thread not found');
			}

			if (!data.user) {
				goto('/login?error=not_moderator');
				return;
			}

			const response = await fetch(`/api/threads/${data.thread.id}/notes`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					author_id: data.user.id,
					author_tag: data.user.username,
					content: newNoteContent.trim()
				})
			});

			if (!response.ok) {
				throw new Error('Failed to add note');
			}

			success = 'Internal note added successfully!';
			newNoteContent = '';
			await invalidateAll(); // Refresh server data
		} catch (err) {
			error = 'Failed to add note';
			console.error('Error adding note:', err);
		} finally {
			loading = false;
		}
	}

	async function closeThread() {
		try {
			loading = true;
			error = '';

			if (!data.thread) {
				throw new Error('Thread not found');
			}

			const response = await fetch(`/api/threads/${data.thread.id}/close`, {
				method: 'POST'
			});

			if (!response.ok) {
				throw new Error('Failed to close thread');
			}

			success = 'Thread closed successfully!';
			await invalidateAll(); // Refresh server data
		} catch (err) {
			error = 'Failed to close thread';
			console.error('Error closing thread:', err);
		} finally {
			loading = false;
		}
	}

	function formatUserId(userId: string) {
		return `${userId.slice(0, 4)}...${userId.slice(-4)}`;
	}

	function clearMessages() {
		error = '';
		success = '';
	}

	function goToPage(pageNum: number) {
		const url = new URL(window.location.href);
		url.searchParams.set('page', pageNum.toString());
		goto(url.pathname + url.search);
	}
</script>

<svelte:head>
	<title>Thread {data.thread?.id || 'Error fetching thread'} - ModMail</title>
</svelte:head>

<div class="page">
	<div class="page-header">
		<div class="header-left">
			<button onclick={() => goto('/')} class="back-btn">
				<ArrowLeft size={16} />
				Back to Threads
			</button>
			<div class="thread-info">
				<h1>Thread #{data.thread?.id || 'Error fetching thread'}</h1>
				<div class="thread-meta">
					<div class="meta-item">
						<User size={14} />
						<span>User: {formatUserId(data.thread?.user_id || 'Error fetching thread')}</span>
					</div>
					<div class="meta-item">
						<MessageCircle size={14} />
						<span>Channel: {data.thread?.thread_id.slice(0, 8) || 'Error fetching thread'}...</span>
					</div>
					<div
						class="thread-status"
						class:open={data.thread?.is_open}
						class:closed={!data.thread?.is_open}
					>
						{data.thread?.is_open ? 'Open' : 'Closed'}
					</div>
				</div>
			</div>
		</div>
		<div class="header-actions">
			{#if data.thread?.is_open}
				<button onclick={closeThread} class="close-btn" disabled={loading}>
					<XCircle size={16} />
					Close Thread
				</button>
			{/if}
		</div>
	</div>

	{#if error}
		<div class="alert alert-error">
			{error}
			<button class="alert-close" onclick={clearMessages}>×</button>
		</div>
	{/if}

	{#if success}
		<div class="alert alert-success">
			{success}
			<button class="alert-close" onclick={clearMessages}>×</button>
		</div>
	{/if}

	<div class="content">
		<div class="messages-section">
			<div class="section-header">
				<h2>Messages ({data.pagination?.total_count || data.messages?.length || 0})</h2>
			</div>

			{#if data.messages?.length === 0}
				<div class="empty-state">
					<MessageCircle size={48} color="#ccc" />
					<h3>No messages yet</h3>
					<p>Messages in this thread will appear here.</p>
				</div>
			{:else}
				<div class="messages-list">
					{#if data.messages}
						{#each data.messages as message (message.id)}
							<div class="message-card">
								<div class="message-header">
									<div class="author-info">
										<User size={16} />
										<span class="author-tag">{message.author_tag}</span>
										<span class="author-id">({message.author_id.slice(0, 8)}...)</span>
									</div>
									<div class="timestamp">
										<Clock size={14} />
										<span>{formatDate(message.created_at)}</span>
									</div>
								</div>
								<div class="message-content">
									{message.content}
								</div>
								{#if message.attachments && message.attachments.length > 0}
									<div class="attachments">
										<h4>Attachments:</h4>
										<div class="attachment-list">
											{#each message.attachments as attachment}
												<div class="attachment-item">
													{#if attachment.content_type?.startsWith('image/')}
														<img
															src={attachment.url}
															alt={attachment.filename}
															class="attachment-image"
														/>
													{:else}
														<a
															href={attachment.url}
															target="_blank"
															rel="noopener noreferrer"
															class="attachment-link"
														>
															📎 {attachment.filename}
														</a>
													{/if}
													<div class="attachment-info">
														<span class="filename">{attachment.filename}</span>
														<span class="filesize">({formatFileSize(attachment.size)} KB)</span>
													</div>
												</div>
											{/each}
										</div>
									</div>
								{/if}
								<div class="message-footer">
									<span class="message-id">ID: {message.id.slice(0, 8)}...</span>
								</div>
							</div>
						{/each}
					{/if}
				</div>

				{#if data.pagination && data.pagination.total_pages > 1}
					<div class="pagination">
						<div class="pagination-info">
							Showing {(data.pagination.page - 1) * data.pagination.limit + 1} to {Math.min(
								data.pagination.page * data.pagination.limit,
								data.pagination.total_count
							)} of {data.pagination.total_count} messages
						</div>
						<div class="pagination-controls">
							<button
								onclick={() => goToPage(data.pagination.page - 1)}
								class="pagination-btn"
								disabled={!data.pagination.has_prev}
							>
								<ChevronLeft size={16} />
								Previous
							</button>

							{#each Array.from({ length: Math.min(5, data.pagination.total_pages) }, (_, i) => {
								const start = Math.max(1, data.pagination.page - 2);
								const end = Math.min(data.pagination.total_pages, start + 4);
								return start + i;
							}).filter((p) => p <= data.pagination.total_pages) as pageNum}
								<button
									onclick={() => goToPage(pageNum)}
									class="pagination-btn page-btn"
									class:active={pageNum === data.pagination.page}
								>
									{pageNum}
								</button>
							{/each}

							<button
								onclick={() => goToPage(data.pagination.page + 1)}
								class="pagination-btn"
								disabled={!data.pagination.has_next}
							>
								Next
								<ChevronRight size={16} />
							</button>
						</div>
					</div>
				{/if}
			{/if}
		</div>

		<div class="notes-section">
			<div class="section-header">
				<h2>Internal Notes ({data.notes?.length || 0})</h2>
			</div>

			{#if !data.notes || data.notes.length === 0}
				<div class="empty-state">
					<StickyNote size={48} color="#ccc" />
					<h3>No internal notes yet</h3>
					<p>Internal moderator notes will appear here.</p>
				</div>
			{:else}
				<div class="notes-list">
					{#each data.notes as note (note.id)}
						<div class="note-card">
							<div class="note-header">
								<div class="author-info">
									<StickyNote size={16} />
									<span class="author-tag">{note.author_tag}</span>
									<span class="author-id">({note.author_id.slice(0, 8)}...)</span>
								</div>
								<div class="timestamp">
									<Clock size={14} />
									<span>{formatDate(note.created_at)}</span>
								</div>
							</div>
							<div class="note-content">
								{note.content}
							</div>
							<div class="note-footer">
								<span class="note-id">ID: {note.id.slice(0, 8)}...</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		{#if data.thread?.is_open}
			<div class="add-note-section">
				<div class="section-header">
					<h2>Add Internal Note</h2>
				</div>
				<form onsubmit={addNote} class="note-form">
					<div class="form-group">
						<label for="content">Note:</label>
						<textarea
							id="content"
							bind:value={newNoteContent}
							placeholder="Enter internal note content..."
							rows="3"
							required
						></textarea>
					</div>
					<div class="form-actions">
						<button type="submit" class="send-btn" disabled={loading}>
							<StickyNote size={16} />
							{loading ? 'Adding...' : 'Add Note'}
						</button>
					</div>
				</form>
			</div>
		{/if}
	</div>
</div>

<style>
	.page {
		max-width: 1000px;
		margin: 0 auto;
		padding: 2rem;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		gap: 2rem;
	}

	.header-left {
		flex: 1;
	}

	.back-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: #6b7280;
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.9rem;
		margin-bottom: 1rem;
		transition: background-color 0.2s;
	}

	.back-btn:hover {
		background: #4b5563;
	}

	.thread-info h1 {
		margin: 0 0 0.5rem 0;
		color: #2c2f36;
		font-size: 1.75rem;
		font-weight: 600;
	}

	.thread-meta {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		flex-wrap: wrap;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #666;
		font-size: 0.9rem;
	}

	.thread-status {
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.85rem;
		font-weight: 500;
		text-transform: uppercase;
	}

	.thread-status.open {
		background: #d4edda;
		color: #155724;
	}

	.thread-status.closed {
		background: #f8d7da;
		color: #721c24;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.close-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: #ed4245;
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: background-color 0.2s;
	}

	.close-btn:hover:not(:disabled) {
		background: #c73e3e;
	}

	.close-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.alert {
		padding: 1rem;
		border-radius: 0.5rem;
		margin-bottom: 1rem;
		position: relative;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.alert-error {
		background-color: #fee;
		color: #c53030;
		border: 1px solid #feb2b2;
	}

	.alert-success {
		background-color: #f0fff4;
		color: #2f855a;
		border: 1px solid #9ae6b4;
	}

	.alert-close {
		background: none;
		border: none;
		font-size: 1.2rem;
		cursor: pointer;
		padding: 0;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.content {
		display: grid;
		gap: 2rem;
	}

	.section-header {
		margin-bottom: 1rem;
	}

	.section-header h2 {
		margin: 0;
		color: #374151;
		font-size: 1.25rem;
		font-weight: 600;
	}

	.empty-state {
		text-align: center;
		padding: 3rem;
		color: #666;
	}

	.empty-state h3 {
		margin: 1rem 0 0.5rem 0;
		color: #374151;
	}

	.empty-state p {
		margin: 0;
		color: #999;
	}

	.messages-list,
	.notes-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.message-card,
	.note-card {
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 1.25rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.note-card {
		background: #fffbf0;
		border-color: #f59e0b;
		border-left: 4px solid #f59e0b;
	}

	.message-header,
	.note-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.author-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #666;
	}

	.author-tag {
		font-weight: 600;
		color: #2c2f36;
	}

	.author-id {
		font-family: 'Monaco', 'Menlo', monospace;
		font-size: 0.85rem;
		background: #f5f5f5;
		padding: 0.2rem 0.4rem;
		border-radius: 4px;
	}

	.timestamp {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		color: #999;
		font-size: 0.85rem;
	}

	.message-content,
	.note-content {
		margin-bottom: 0.75rem;
		line-height: 1.5;
		color: #2c2f36;
		white-space: pre-wrap;
		word-wrap: break-word;
	}

	.message-footer {
		display: flex;
		justify-content: flex-end;
	}

	.message-id,
	.note-id {
		font-family: 'Monaco', 'Menlo', monospace;
		font-size: 0.8rem;
		color: #999;
		background: #f8f9fa;
		padding: 0.2rem 0.5rem;
		border-radius: 4px;
	}

	.add-note-section {
		background: #fffbf0;
		border: 1px solid #f59e0b;
		border-radius: 8px;
		padding: 1.5rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.note-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-weight: 500;
		color: #374151;
	}

	.form-group textarea {
		padding: 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 1rem;
	}

	.form-group textarea:focus {
		outline: none;
		border-color: #f59e0b;
		box-shadow: 0 0 0 3px rgba(245, 158, 11, 0.1);
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
	}

	.send-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: #f59e0b;
		color: white;
		border: none;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: background-color 0.2s;
	}

	.send-btn:hover:not(:disabled) {
		background: #d97706;
	}

	.send-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination {
		margin-top: 2rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		align-items: center;
	}

	.pagination-info {
		color: #666;
		font-size: 0.9rem;
	}

	.pagination-controls {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.pagination-btn {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		background: white;
		border: 1px solid #e0e0e0;
		padding: 0.5rem 0.75rem;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.9rem;
		transition: all 0.2s;
	}

	.pagination-btn:hover:not(:disabled) {
		background: #f5f5f5;
		border-color: #ccc;
	}

	.pagination-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-btn.active {
		background: #5865f2;
		color: white;
		border-color: #5865f2;
	}

	.page-btn.active:hover {
		background: #4752c4;
		border-color: #4752c4;
	}

	@media (max-width: 768px) {
		.page {
			padding: 1rem;
		}

		.page-header {
			flex-direction: column;
			align-items: stretch;
		}

		.thread-meta {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
		}

		.message-header,
		.note-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
		}

		.pagination-controls {
			flex-wrap: wrap;
			justify-content: center;
		}
	}

	.attachments {
		margin: 0.75rem 0;
		padding: 0.75rem;
		background: #f8f9fa;
		border-radius: 6px;
		border: 1px solid #e9ecef;
	}

	.attachments h4 {
		margin: 0 0 0.5rem 0;
		font-size: 0.9rem;
		color: #495057;
		font-weight: 600;
	}

	.attachment-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.attachment-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.attachment-image {
		max-width: 300px;
		max-height: 200px;
		border-radius: 4px;
		border: 1px solid #dee2e6;
	}

	.attachment-link {
		color: #0066cc;
		text-decoration: none;
		font-weight: 500;
	}

	.attachment-link:hover {
		text-decoration: underline;
	}

	.attachment-info {
		display: flex;
		gap: 0.5rem;
		font-size: 0.85rem;
		color: #6c757d;
	}

	.filename {
		font-weight: 500;
	}

	.filesize {
		color: #adb5bd;
	}
</style>
