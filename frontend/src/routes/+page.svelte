<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { goto } from '$app/navigation';
	import { Clock, MessageCircle, User, XCircle, ChevronLeft, ChevronRight } from 'lucide-svelte';
	import type { Thread } from '$lib/api';
	import type { PageProps } from './$types';
	import { api } from '$lib/api';
	import { enhance } from '$app/forms';

	let { data }: PageProps = $props();

	let loading = $state(false);
	let error = $state('');

	// Handle server errors
	$effect.pre(() => {
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

	async function refreshThreads() {
		try {
			loading = true;
			error = '';
			await invalidateAll(); // Refresh server data
		} catch (err) {
			error = 'Failed to refresh threads';
			console.error(err);
		} finally {
			loading = false;
		}
	}

	function formatUserId(userId: string) {
		return `${userId.slice(0, 4)}...${userId.slice(-4)}`;
	}

	function goToPage(pageNum: number) {
		const url = new URL(window.location.href);
		url.searchParams.set('page', pageNum.toString());
		goto(url.pathname + url.search);
	}
</script>

<svelte:head>
	<title>Threads - Modmail Dashboard</title>
</svelte:head>

<div class="page">
	<div class="page-header">
		<h1>Modmail Threads</h1>
		<button onclick={refreshThreads} class="refresh-btn" disabled={loading}>
			{loading ? 'Loading...' : 'Refresh'}
		</button>
	</div>

	{#if error}
		<div class="error">{error}</div>
	{:else if loading && data.threads.length === 0}
		<div class="loading">Loading threads...</div>
	{:else if data.threads.length === 0}
		<div class="empty-state">
			<MessageCircle size={48} color="#ccc" />
			<h3>No threads found</h3>
			<p>When users send DMs to the bot, threads will appear here.</p>
		</div>
	{:else}
		<div class="threads-grid">
			{#each data.threads as thread (thread.id)}
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
							<form
								method="POST"
								action="?/closeThread"
								use:enhance={() => {
									loading = true;
									return async ({ result }) => {
										loading = false;
										if (result.type === 'success') {
											await invalidateAll();
										}
									};
								}}
							>
								<input type="hidden" name="id" value={thread.id} />
								<button type="submit" class="close-btn" disabled={loading}>
									<XCircle size={16} />
									Close
								</button>
							</form>
						{/if}
					</div>
				</div>
			{/each}
		</div>

		{#if data.pagination && data.pagination.total_pages > 1}
			<div class="pagination">
				<div class="pagination-info">
					Showing {(data.pagination.page - 1) * data.pagination.limit + 1} to {Math.min(
						data.pagination.page * data.pagination.limit,
						data.pagination.total_count
					)} of {data.pagination.total_count} threads
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

<style lang="stylus">
	@import '../styles/_variables.styl'
	@import '../styles/_mixins.styl'

	.page
		container()

	.page-header
		flex-between()
		margin-bottom spacing-2xl

		h1
			margin 0
			color text-primary
			font-size font-size-4xl
			font-weight font-weight-semibold

		+mobile()
			flex-direction column
			gap spacing-lg
			align-items stretch

	.refresh-btn
		button-base()
		button-size()
		button-variant(primary, primary-hover)

	.loading,
	.error
		text-align center
		padding spacing-3xl
		color text-secondary
		font-size font-size-lg

	.error
		color danger

	.empty-state
		text-align center
		padding spacing-4xl spacing-2xl

		h3
			margin spacing-lg 0 spacing-sm 0
			color text-secondary
			font-size font-size-xl

		p
			margin 0
			color text-muted

	.threads-grid
		grid-auto-fill(350px, spacing-xl)

		+mobile()
			grid-template-columns 1fr

	.thread-card
		card-base()
		card-hover()
		card-padding()

		&.closed
			opacity 0.7
			border-color text-light

	.thread-header
		flex-between()
		margin-bottom spacing-lg

	.thread-info
		flex-center()
		gap spacing-sm
		color text-secondary

	.user-id
		font-family font-family-mono
		font-size font-size-base
		background bg-light
		padding spacing-xs spacing-sm
		border-radius radius-sm

	.thread-status
		status-badge(transparent, inherit)

		&.open
			status-badge(#d4edda, #155724)

		&.closed
			status-badge(#f8d7da, #721c24)

	.thread-details
		margin-bottom spacing-xl

	.detail-item
		flex-center()
		gap spacing-sm
		color text-secondary
		font-size font-size-base
		margin-bottom spacing-sm

		&:last-child
			margin-bottom 0

	.thread-actions
		display flex
		gap spacing-md

	.view-btn
		button-base()
		button-size()
		button-variant(success, success-hover)
		flex 1

	.close-btn
		button-base()
		button-size()
		button-variant(danger, danger-hover)
		gap spacing-sm

	.pagination
		margin-top spacing-2xl
		flex-column()
		gap spacing-lg
		align-items center

	.pagination-info
		color text-secondary
		font-size font-size-base

	.pagination-controls
		display flex
		gap spacing-sm
		align-items center

		+mobile()
			flex-wrap wrap
			justify-content center

	.pagination-btn
		button-base()
		button-size(spacing-sm, spacing-md, font-size-base)
		button-variant(bg-white, bg-light, text-primary)
		border 1px solid border-light
		gap spacing-xs

		&:hover:not(:disabled)
			border-color text-light

		&:disabled
			opacity 0.5
			cursor not-allowed

		&.page-btn.active
			button-variant(primary, primary-hover)
</style>
