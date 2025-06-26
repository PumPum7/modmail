<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { Clock, MessageCircle, User, XCircle } from 'lucide-svelte';
	import type { Thread } from '$lib/api';
	import { api } from '$lib/api';

	let { data } = $props();

	let threads: Thread[] = $state([]);
	let loading = $state(true);
	let error = $state('');

	onMount(async () => {
		if (!data.user) {
			goto('/login');
			return;
		}
		
		if (!data.user.isModerator) {
			goto('/login?error=not_moderator');
			return;
		}

		await loadThreads();
	});

	async function loadThreads() {
		try {
			loading = true;
			threads = await api.getAllThreads();
		} catch (err) {
			error = 'Failed to load threads';
			console.error(err);
		} finally {
			loading = false;
		}
	}

	async function closeThread(thread: Thread) {
		try {
			await api.closeThread(thread.id);
			await loadThreads(); // Reload threads
		} catch (err) {
			console.error('Failed to close thread:', err);
		}
	}

	function formatUserId(userId: string) {
		return `${userId.slice(0, 4)}...${userId.slice(-4)}`;
	}
</script>

<svelte:head>
	<title>Threads - Modmail Dashboard</title>
</svelte:head>

<div class="page">
	<div class="page-header">
		<h1>Modmail Threads</h1>
		<button onclick={loadThreads} class="refresh-btn">Refresh</button>
	</div>

	{#if loading}
		<div class="loading">Loading threads...</div>
	{:else if error}
		<div class="error">{error}</div>
	{:else if threads.length === 0}
		<div class="empty-state">
			<MessageCircle size={48} color="#ccc" />
			<h3>No threads found</h3>
			<p>When users send DMs to the bot, threads will appear here.</p>
		</div>
	{:else}
		<div class="threads-grid">
			{#each threads as thread (thread.id)}
				<div class="thread-card" class:closed={!thread.is_open}>
					<div class="thread-header">
						<div class="thread-info">
							<User size={16} />
							<span class="user-id">{formatUserId(thread.user_id)}</span>
						</div>
						<div class="thread-status" class:open={thread.is_open} class:closed={!thread.is_open}>
							{thread.is_open ? 'Open' : 'Closed'}
						</div>
					</div>

					<div class="thread-details">
						<div class="detail-item">
							<Clock size={14} />
							<span>Thread ID: {thread.id}</span>
						</div>
						<div class="detail-item">
							<MessageCircle size={14} />
							<span>Channel: {thread.thread_id.slice(0, 8)}...</span>
						</div>
					</div>

					<div class="thread-actions">
						<button onclick={() => goto(`/thread/${thread.id}`)} class="view-btn">
							View Messages
						</button>
						{#if thread.is_open}
							<button onclick={() => closeThread(thread)} class="close-btn">
								<XCircle size={16} />
								Close
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.page {
		max-width: 1200px;
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

	.threads-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
		gap: 1.5rem;
	}

	.thread-card {
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 12px;
		padding: 1.5rem;
		transition: all 0.2s;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.thread-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.thread-card.closed {
		opacity: 0.7;
		border-color: #ccc;
	}

	.thread-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.thread-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #666;
	}

	.user-id {
		font-family: 'Monaco', 'Menlo', monospace;
		font-size: 0.9rem;
		background: #f5f5f5;
		padding: 0.2rem 0.5rem;
		border-radius: 4px;
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

	.thread-details {
		margin-bottom: 1.5rem;
	}

	.detail-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #666;
		font-size: 0.9rem;
		margin-bottom: 0.5rem;
	}

	.detail-item:last-child {
		margin-bottom: 0;
	}

	.thread-actions {
		display: flex;
		gap: 0.75rem;
	}

	.view-btn {
		flex: 1;
		background: #28a745;
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: background-color 0.2s;
	}

	.view-btn:hover {
		background: #218838;
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

	.close-btn:hover {
		background: #c73e3e;
	}

	@media (max-width: 768px) {
		.threads-grid {
			grid-template-columns: 1fr;
		}
		
		.page-header {
			flex-direction: column;
			gap: 1rem;
			align-items: stretch;
		}
	}
</style> 