<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { goto } from '$app/navigation';
	import { Clock, MessageCircle, User, Send, ArrowLeft, XCircle } from 'lucide-svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let loading = $state(false);
	let error = $state('');
	let success = $state('');
	let newMessageContent = $state('');
	let newMessageAuthor = $state('');

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

	async function addMessage() {
		if (!newMessageContent.trim() || !newMessageAuthor.trim()) {
			error = 'Both author and message content are required';
			return;
		}

		try {
			loading = true;
			error = '';
			
			const response = await fetch(`/api/threads/${data.thread.id}/messages`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					author_id: '123456789', // This would typically come from the authenticated user
					author_tag: newMessageAuthor.trim(),
					content: newMessageContent.trim()
				})
			});

			if (!response.ok) {
				throw new Error('Failed to add message');
			}

			success = 'Message added successfully!';
			newMessageContent = '';
			newMessageAuthor = '';
			await invalidateAll(); // Refresh server data
		} catch (err) {
			error = 'Failed to add message';
			console.error('Error adding message:', err);
		} finally {
			loading = false;
		}
	}

	async function closeThread() {
		try {
			loading = true;
			error = '';
			
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

	function formatDate(dateString: string) {
		const date = new Date(dateString);
		return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
	}

	function formatUserId(userId: string) {
		return `${userId.slice(0, 4)}...${userId.slice(-4)}`;
	}

	function clearMessages() {
		error = '';
		success = '';
	}
</script>

<svelte:head>
	<title>Thread {data.thread.id} - ModMail</title>
</svelte:head>

<div class="page">
	<div class="page-header">
		<div class="header-left">
			<button onclick={() => goto('/')} class="back-btn">
				<ArrowLeft size={16} />
				Back to Threads
			</button>
			<div class="thread-info">
				<h1>Thread #{data.thread.id}</h1>
				<div class="thread-meta">
					<div class="meta-item">
						<User size={14} />
						<span>User: {formatUserId(data.thread.user_id)}</span>
					</div>
					<div class="meta-item">
						<MessageCircle size={14} />
						<span>Channel: {data.thread.thread_id.slice(0, 8)}...</span>
					</div>
					<div class="thread-status" class:open={data.thread.is_open} class:closed={!data.thread.is_open}>
						{data.thread.is_open ? 'Open' : 'Closed'}
					</div>
				</div>
			</div>
		</div>
		<div class="header-actions">
			{#if data.thread.is_open}
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
				<h2>Messages ({data.messages.length})</h2>
			</div>

			{#if data.messages.length === 0}
				<div class="empty-state">
					<MessageCircle size={48} color="#ccc" />
					<h3>No messages yet</h3>
					<p>Messages in this thread will appear here.</p>
				</div>
			{:else}
				<div class="messages-list">
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
							<div class="message-footer">
								<span class="message-id">ID: {message.id.slice(0, 8)}...</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		{#if data.thread.is_open}
			<div class="add-message-section">
				<div class="section-header">
					<h2>Add Message</h2>
				</div>
				<form onsubmit={addMessage} class="message-form">
					<div class="form-row">
						<div class="form-group">
							<label for="author">Author:</label>
							<input
								id="author"
								type="text"
								bind:value={newMessageAuthor}
								placeholder="Enter author name..."
								required
							/>
						</div>
					</div>
					<div class="form-group">
						<label for="content">Message:</label>
						<textarea
							id="content"
							bind:value={newMessageContent}
							placeholder="Enter message content..."
							rows="3"
							required
						></textarea>
					</div>
					<div class="form-actions">
						<button type="submit" class="send-btn" disabled={loading}>
							<Send size={16} />
							{loading ? 'Sending...' : 'Send Message'}
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

	.messages-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.message-card {
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 1.25rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.message-header {
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

	.message-content {
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

	.message-id {
		font-family: 'Monaco', 'Menlo', monospace;
		font-size: 0.8rem;
		color: #999;
		background: #f8f9fa;
		padding: 0.2rem 0.5rem;
		border-radius: 4px;
	}

	.add-message-section {
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 1.5rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.message-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr;
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

	.form-group input,
	.form-group textarea {
		padding: 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 1rem;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #4f46e5;
		box-shadow: 0 0 0 3px rgba(79, 70, 229, 0.1);
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
	}

	.send-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: #28a745;
		color: white;
		border: none;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: background-color 0.2s;
	}

	.send-btn:hover:not(:disabled) {
		background: #218838;
	}

	.send-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
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

		.form-row {
			grid-template-columns: 1fr;
		}

		.message-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
		}
	}
</style> 