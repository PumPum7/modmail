<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { goto } from '$app/navigation';
	import { enhance } from '$app/forms';
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
	import { api } from '$lib/api';

	let { data, form }: PageProps = $props();

	let loading = $state(false);
	let error = $state('');
	let success = $state('');
	let newNoteContent = $state('');
	let editingUrgency = $state(false);
	let selectedUrgency = $state('');

	// Handle server errors
	$effect.pre(() => {
		if (data.error) {
			error = data.error;
		}
		if (data.thread) {
			selectedUrgency = data.thread.urgency;
		}
	});

	// Handle form results
	$effect(() => {
		if (form?.success) {
			success = form.success as string;
			error = '';
			newNoteContent = '';
		} else if (form?.error) {
			error = form.error as string;
			success = '';
		}
		loading = false;
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

	function getUrgencyColor(urgency: string) {
		switch (urgency?.toLowerCase()) {
			case 'urgent':
				return '#dc2626'; // red-600
			case 'high':
				return '#ea580c'; // orange-600
			case 'medium':
				return '#ca8a04'; // yellow-600
			case 'low':
				return '#16a34a'; // green-600
			default:
				return '#6b7280'; // gray-500
		}
	}

	function getUrgencyBgColor(urgency: string) {
		switch (urgency?.toLowerCase()) {
			case 'urgent':
				return '#fee2e2'; // red-100
			case 'high':
				return '#fed7aa'; // orange-100
			case 'medium':
				return '#fef3c7'; // yellow-100
			case 'low':
				return '#dcfce7'; // green-100
			default:
				return '#f3f4f6'; // gray-100
		}
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
					<div class="meta-item urgency-item">
						{#if editingUrgency}
							<form
								method="POST"
								action="?/updateUrgency"
								use:enhance={() => {
									loading = true;
									return async ({ result }) => {
										if (result.type === 'success') {
											editingUrgency = false;
											await invalidateAll();
										}
										loading = false;
									};
								}}
							>
								<select name="urgency" bind:value={selectedUrgency} class="urgency-select" required>
									<option value="Low">Low</option>
									<option value="Medium">Medium</option>
									<option value="High">High</option>
									<option value="Urgent">Urgent</option>
								</select>
								<button type="submit" class="urgency-save-btn" disabled={loading}> Save </button>
								<button
									type="button"
									onclick={() => {
										editingUrgency = false;
										selectedUrgency = data.thread?.urgency || 'Medium';
									}}
									class="urgency-cancel-btn"
								>
									Cancel
								</button>
							</form>
						{:else}
							<span>Priority:</span>
							<div
								class="urgency-badge"
								style="color: {getUrgencyColor(
									data.thread?.urgency || 'Medium'
								)}; background-color: {getUrgencyBgColor(data.thread?.urgency || 'Medium')}"
							>
								{data.thread?.urgency || 'Medium'}
							</div>
							{#if data.thread?.is_open && data.user?.isModerator}
								<button onclick={() => (editingUrgency = true)} class="urgency-edit-btn">
									Edit
								</button>
							{/if}
						{/if}
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
					<input type="hidden" name="id" value={data.thread?.id} />
					<button type="submit" class="close-btn" disabled={loading}>
						<XCircle size={16} />
						Close Thread
					</button>
				</form>
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
				<form
					method="POST"
					action="?/addNote"
					use:enhance={() => {
						loading = true;
						clearMessages();
						return async ({ result }) => {
							loading = false;
							if (result.type === 'success') {
								await invalidateAll();
							}
						};
					}}
					class="note-form"
				>
					<div class="form-group">
						<label for="content">Note:</label>
						<textarea
							id="content"
							bind:value={newNoteContent}
							placeholder="Enter internal note content..."
							rows="3"
							required
							name="content"
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

<style lang="stylus">
	@import '../../../styles/_variables.styl'
	@import '../../../styles/_mixins.styl'

	.page
		max-width 1000px
		margin 0 auto
		padding spacing-2xl

		+mobile()
			padding spacing-lg

	.page-header
		flex-between()
		align-items flex-start
		margin-bottom spacing-2xl
		gap spacing-2xl

		+mobile()
			flex-direction column
			align-items stretch

	.header-left
		flex 1

	.back-btn
		button-base()
		button-size(spacing-sm, spacing-lg, font-size-base)
		button-variant(secondary, secondary-hover)
		gap spacing-sm
		margin-bottom spacing-lg

	.thread-info h1
		margin 0 0 spacing-sm 0
		color text-primary
		font-size font-size-3xl
		font-weight font-weight-semibold

	.thread-meta
		display flex
		align-items center
		gap spacing-xl
		flex-wrap wrap

		+mobile()
			flex-direction column
			align-items flex-start
			gap spacing-sm

	.meta-item
		flex-center()
		gap spacing-sm
		color text-secondary
		font-size font-size-base

	.thread-status
		status-badge(transparent, inherit)

		&.open
			status-badge(#d4edda, #155724)

		&.closed
			status-badge(#f8d7da, #721c24)

	.close-btn
		button-base()
		button-size()
		button-variant(danger, danger-hover)
		gap spacing-sm

	.alert
		alert-base()

		&.alert-error
			alert-variant(#fee, #c53030, #feb2b2)

		&.alert-success
			alert-variant(#f0fff4, #2f855a, #9ae6b4)

	.alert-close
		background none
		border none
		font-size font-size-xl
		cursor pointer
		padding 0
		width 24px
		height 24px
		flex-center()

	.content
		flex-column()
		gap spacing-2xl

	.section-header
		margin-bottom spacing-lg

		h2
			margin 0
			color text-primary
			font-size font-size-xl
			font-weight font-weight-semibold

	.empty-state
		text-align center
		padding spacing-3xl
		color text-secondary

		h3
			margin spacing-lg 0 spacing-sm 0
			color text-primary

		p
			margin 0
			color text-muted

	.messages-list,
	.notes-list
		flex-column()
		gap spacing-lg

	.message-card,
	.note-card
		card-base()
		card-padding()

	.note-card
		background bg-warning-light
		border-color warning
		border-left 4px solid warning

	.message-header,
	.note-header
		+mobile()
			flex-direction column
			align-items flex-start
			gap spacing-sm

		flex-between()
		& 
			margin-bottom spacing-md

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

	.message-content,
	.note-content
		margin-bottom spacing-md
		line-height line-height-normal
		color text-primary
		white-space pre-wrap
		word-wrap break-word

	.message-footer
		display flex
		justify-content flex-end

	.message-id,
	.note-id
		font-family font-family-mono
		font-size font-size-xs
		color text-muted
		background bg-gray
		padding spacing-xs spacing-sm
		border-radius radius-sm

	.add-note-section
		background bg-warning-light
		border 1px solid warning
		border-radius radius-lg
		padding spacing-xl
		box-shadow shadow-md

	.note-form
		flex-column()
		gap spacing-lg

	.form-group
		flex-column()
		gap spacing-sm

		label
			form-label()

		textarea
			form-input()

	.form-actions
		display flex
		justify-content flex-end

	.send-btn
		button-base()
		button-size(spacing-md, spacing-xl)
		button-variant(warning, warning-hover)
		gap spacing-sm

	// Pagination styles (reused from main page)
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

	// Attachment styles
	.attachments
		margin spacing-md 0
		padding spacing-md
		background bg-gray
		border-radius radius-md
		border 1px solid border-muted

		h4
			margin 0 0 spacing-sm 0
			font-size font-size-base
			color text-primary
			font-weight font-weight-semibold

	.attachment-list
		flex-column()
		gap spacing-sm

	.attachment-item
		flex-column()
		gap spacing-xs

	.attachment-image
		max-width 300px
		max-height 200px
		border-radius radius-sm
		border 1px solid border-muted

	.attachment-link
		color primary
		text-decoration none
		font-weight font-weight-medium

		&:hover
			text-decoration underline

	.attachment-info
		display flex
		gap spacing-sm
		font-size font-size-sm
		color text-secondary

	.filename
		font-weight font-weight-medium

	.filesize
		color text-muted

	// Urgency editing styles
	.urgency-item
		flex-center()
		gap spacing-sm

	.urgency-badge
		status-badge(transparent, inherit)
		text-transform capitalize
		border 1px solid currentColor

	.urgency-edit-btn
		background none
		border 1px solid border-gray
		color text-secondary
		padding spacing-xs spacing-sm
		border-radius radius-sm
		cursor pointer
		font-size font-size-xs
		transition all transition-fast

		&:hover
			background bg-light
			border-color text-muted

	.urgency-select
		padding spacing-xs spacing-sm
		border 1px solid border-gray
		border-radius radius-sm
		font-size font-size-sm

	.urgency-save-btn
		button-base()
		button-size(spacing-xs, spacing-md, font-size-xs)
		button-variant(success, success-hover)

	.urgency-cancel-btn
		button-base()
		button-size(spacing-xs, spacing-md, font-size-xs)
		button-variant(secondary, secondary-hover)
</style>
