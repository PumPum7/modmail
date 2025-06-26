<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { goto } from '$app/navigation';
	import { Search, MessageSquare, Clock, User } from 'lucide-svelte';
	import type { Message } from '$lib/api';
	import type { PageProps } from './$types';
	import { formatDate, truncateContent } from '$lib/util';

	let { data }: PageProps = $props();

	let filteredMessages: Message[] = $state([]);
	let loading = $state(false);
	let error = $state('');
	let searchTerm = $state('');

	// Initialize filtered messages with server data
	$effect.pre(() => {
		if (data.messages) {
			filteredMessages = [...data.messages];
		}
		if (data.error) {
			error = data.error;
		}
	});

	// Handle authentication on mount (client-side only)
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

	async function refreshMessages() {
		try {
			loading = true;
			error = '';
			await invalidateAll(); // Refresh server data
		} catch (err) {
			error = 'Failed to refresh messages';
			console.error(err);
		} finally {
			loading = false;
		}
	}

	function filterMessages() {
		if (!data.messages) return;
		
		if (!searchTerm.trim()) {
			filteredMessages = [...data.messages];
			return;
		}

		const term = searchTerm.toLowerCase();
		filteredMessages = data.messages.filter((message: Message) =>
			message.content.toLowerCase().includes(term) ||
			message.author_tag.toLowerCase().includes(term) ||
			message.author_id.includes(term)
		);
	}

	$effect.pre(() => {
		if (searchTerm !== undefined) {
			filterMessages();
		}
	});
</script>

<svelte:head>
	<title>Messages - Modmail Dashboard</title>
</svelte:head>

<div class="page">
	<div class="page-header">
		<h1>All Messages</h1>
		<button onclick={refreshMessages} class="refresh-btn" disabled={loading}>
			{loading ? 'Loading...' : 'Refresh'}
		</button>
	</div>

	<div class="search-bar">
		<Search size={20} />
		<input
			type="text"
			placeholder="Search messages, authors, or user IDs..."
			bind:value={searchTerm}
			class="search-input"
		/>
	</div>

	{#if error}
		<div class="error">{error}</div>
	{:else if loading && data.messages.length === 0}
		<div class="loading">Loading messages...</div>
	{:else if filteredMessages.length === 0}
		<div class="empty-state">
			<MessageSquare size={48} color="#ccc" />
			<h3>No messages found</h3>
			<p>
				{searchTerm ? 'Try adjusting your search terms.' : 'Messages will appear here as users interact with the bot.'}
			</p>
		</div>
	{:else}
		<div class="messages-list">
			<div class="list-header">
				<div class="count-info">
					Showing {filteredMessages.length} of {data.messages.length} messages
				</div>
			</div>

			{#each filteredMessages as message (message.id)}
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
						{truncateContent(message.content)}
					</div>

					<div class="message-footer">
						<span class="message-id">ID: {message.id.slice(0, 8)}...</span>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.page {
		max-width: 1000px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		margin: 0;
		color: #2c2f36;
		font-size: 2rem;
		font-weight: 600;
	}

	.refresh-btn {
		background: #5865f2;
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: background-color 0.2s;
	}

	.refresh-btn:hover {
		background: #4752c4;
	}

	.search-bar {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 0.75rem 1rem;
		margin-bottom: 2rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.search-input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 1rem;
		background: transparent;
	}

	.search-input::placeholder {
		color: #999;
	}

	.loading,
	.error {
		text-align: center;
		padding: 3rem;
		color: #666;
		font-size: 1.1rem;
	}

	.error {
		color: #ed4245;
	}

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
	}

	.empty-state h3 {
		margin: 1rem 0 0.5rem 0;
		color: #666;
		font-size: 1.25rem;
	}

	.empty-state p {
		margin: 0;
		color: #999;
	}

	.list-header {
		margin-bottom: 1rem;
	}

	.count-info {
		color: #666;
		font-size: 0.9rem;
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
		transition: all 0.2s;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.message-card:hover {
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		transform: translateY(-1px);
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

	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			gap: 1rem;
			align-items: stretch;
		}

		.message-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
		}

		.search-bar {
			padding: 0.5rem 0.75rem;
		}
	}
</style> 