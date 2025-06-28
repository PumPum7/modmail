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
		filteredMessages = data.messages.filter(
			(message: Message) =>
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
				{searchTerm
					? 'Try adjusting your search terms.'
					: 'Messages will appear here as users interact with the bot.'}
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

<style lang="stylus">
	@import '../../styles/_variables.styl'
	@import '../../styles/_mixins.styl'

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

	.search-bar
		flex-center()
		gap spacing-md
		background bg-white
		border 1px solid border-light
		border-radius radius-lg
		padding spacing-md spacing-lg
		margin-bottom spacing-2xl
		box-shadow shadow-sm

	.search-input
		flex 1
		border none
		outline none
		font-size font-size-lg
		background transparent

		&::placeholder
			color text-muted

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

	.list-header
		margin-bottom spacing-lg

	.count-info
		color text-secondary
		font-size font-size-base

	.messages-list
		flex-column()
		gap spacing-lg

	.message-card
		card-base()
		card-hover()
		card-padding(spacing-xl)

	.message-header
		flex-between()
		align-items center
		margin-bottom spacing-md

		+mobile()
			flex-direction column
			align-items flex-start
			gap spacing-sm

	.author-info
		flex-center()
		gap spacing-sm
		color text-secondary

	.author-tag
		font-weight font-weight-semibold
		color text-primary

	.author-id
		font-family font-family-mono
		font-size font-size-sm
		background bg-light
		padding spacing-xs spacing-sm
		border-radius radius-sm

	.timestamp
		flex-center()
		gap spacing-xs
		color text-muted
		font-size font-size-sm

	.message-content
		margin-bottom spacing-md
		line-height line-height-normal
		color text-primary
		white-space pre-wrap
		word-wrap break-word

	.message-footer
		display flex
		justify-content flex-end

	.message-id
		font-family font-family-mono
		font-size font-size-xs
		color text-muted
		background bg-gray
		padding spacing-xs spacing-sm
		border-radius radius-sm
</style>
