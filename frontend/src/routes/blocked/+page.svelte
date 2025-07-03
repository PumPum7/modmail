<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { goto } from '$app/navigation';
	import { enhance } from '$app/forms';
	import { Clock, User, Shield, Plus, Trash2 } from 'lucide-svelte';
	import type { PageProps } from './$types';
	import { formatDate } from '$lib/util';

	let { data, form }: PageProps = $props();

	let loading = $state(false);
	let error = $state('');
	let success = $state('');
	let showBlockForm = $state(false);
	let newBlockUserId = $state('');
	let newBlockUserTag = $state('');
	let newBlockReason = $state('');

	// Handle server errors
	$effect.pre(() => {
		if (data.error) {
			error = data.error;
		}
	});

	// Handle form results
	$effect(() => {
		if (form?.success) {
			success = form.success as string;
			error = '';
			// Reset form state on success
			if ((form.success as string).includes('blocked successfully')) {
				newBlockUserId = '';
				newBlockUserTag = '';
				newBlockReason = '';
				showBlockForm = false;
			}
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

			if (!data.user?.isModerator) {
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
</script>

<svelte:head>
	<title>Blocked Users - ModMail</title>
</svelte:head>

<div class="page">
	<div class="page-header">
		<div class="header-left">
			<button onclick={() => goto('/')} class="back-btn"> ← Back to Threads </button>
			<div class="title-section">
				<h1>Blocked Users</h1>
				<p>Manage users who are blocked from creating modmail threads</p>
			</div>
		</div>
		<div class="header-actions">
			<button onclick={() => (showBlockForm = !showBlockForm)} class="block-btn">
				<Plus size={16} />
				Block User
			</button>
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

	{#if showBlockForm}
		<div class="block-form-section">
			<div class="section-header">
				<h2>Block New User</h2>
			</div>
			<form
				method="POST"
				action="?/block"
				use:enhance={() => {
					loading = true;
					clearMessages();
					return async ({ result }) => {
						if (result.type === 'success') {
							await invalidateAll();
						}
						loading = false;
					};
				}}
				class="block-form"
			>
				<div class="form-row">
					<div class="form-group">
						<label for="userId">User ID:</label>
						<input
							id="userId"
							name="user_id"
							type="text"
							bind:value={newBlockUserId}
							placeholder="Enter Discord user ID..."
							required
						/>
					</div>
					<div class="form-group">
						<label for="userTag">User Tag:</label>
						<input
							id="userTag"
							name="user_tag"
							type="text"
							bind:value={newBlockUserTag}
							placeholder="Enter Discord user tag (e.g., username#1234)..."
							required
						/>
					</div>
				</div>
				<div class="form-group">
					<label for="reason">Reason (optional):</label>
					<textarea
						id="reason"
						bind:value={newBlockReason}
						placeholder="Enter reason for blocking..."
						rows="2"
					></textarea>
				</div>
				<div class="form-actions">
					<button type="button" onclick={() => (showBlockForm = false)} class="cancel-btn">
						Cancel
					</button>
					<button type="submit" class="submit-btn" disabled={loading}>
						<Shield size={16} />
						{loading ? 'Blocking...' : 'Block User'}
					</button>
				</div>
			</form>
		</div>
	{/if}

	<div class="content">
		<div class="section-header">
			<h2>Blocked Users ({data.blockedUsers?.length || 0})</h2>
		</div>

		{#if !data.blockedUsers || data.blockedUsers.length === 0}
			<div class="empty-state">
				<Shield size={48} color="#ccc" />
				<h3>No blocked users</h3>
				<p>Users who are blocked from creating modmail threads will appear here.</p>
			</div>
		{:else}
			<div class="blocked-users-list">
				{#each data.blockedUsers as blockedUser (blockedUser.id)}
					<div class="blocked-user-card">
						<div class="user-header">
							<div class="user-info">
								<User size={16} />
								<span class="user-tag">{blockedUser.user_tag}</span>
								<span class="user-id">({formatUserId(blockedUser.user_id)})</span>
							</div>
							<div class="timestamp">
								<Clock size={14} />
								<span>{formatDate(blockedUser.created_at)}</span>
							</div>
						</div>

						{#if blockedUser.reason}
							<div class="reason">
								<strong>Reason:</strong>
								{blockedUser.reason}
							</div>
						{/if}

						<div class="blocked-by">
							<strong>Blocked by:</strong>
							{blockedUser.blocked_by_tag}
						</div>

						<div class="actions">
							<form
								method="POST"
								action="?/unblock"
								use:enhance={() => {
									loading = true;
									clearMessages();
									return async ({ result }) => {
										if (result.type === 'success') {
											await invalidateAll();
										}
										loading = false;
									};
								}}
							>
								<input type="hidden" name="user_id" value={blockedUser.user_id} />
								<input type="hidden" name="user_tag" value={blockedUser.user_tag} />
								<button class="unblock-btn" disabled={loading}>
									<Trash2 size={16} />
									Unblock
								</button>
							</form>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style lang="stylus">
	@import '../../styles/_variables.styl'
	@import '../../styles/_mixins.styl'

	.page
		container()
		+mobile()
			padding spacing-lg

		& 
			padding-top spacing-2xl
			padding-bottom spacing-2xl

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
		margin-bottom spacing-lg

	.title-section
		h1
			margin 0 0 spacing-sm 0
			color text-primary
			font-size font-size-3xl
			font-weight font-weight-semibold

		p
			margin 0
			color text-secondary
			font-size font-size-base

	.header-actions
		display flex
		gap spacing-lg

	.block-btn
		button-base()
		button-size()
		button-variant(danger, danger-hover)
		gap spacing-sm

	.alert
		alert-base()

		&.alert-error
			alert-variant(#fee, danger, #feb2b2)

		&.alert-success
			alert-variant(#f0fff4, success, #9ae6b4)

	.alert-close
		background none
		border none
		font-size font-size-xl
		cursor pointer
		padding 0
		width 24px
		height 24px
		flex-center()

	.block-form-section
		background lighten(danger, 45%)
		border 1px solid danger
		border-radius radius-lg
		padding spacing-xl
		margin-bottom spacing-2xl
		box-shadow shadow-sm

	.section-header
		margin-bottom spacing-lg

		h2
			margin 0
			color text-primary
			font-size font-size-xl
			font-weight font-weight-semibold

	.block-form
		flex-column()
		gap spacing-lg

	.form-row
		display grid
		grid-template-columns 1fr 1fr
		gap spacing-lg

		+mobile()
			grid-template-columns 1fr

	.form-group
		flex-column()
		gap spacing-sm

		label
			form-label()

		input,
		textarea
			form-input()

	.form-actions
		display flex
		justify-content flex-end
		gap spacing-lg

	.cancel-btn
		button-base()
		button-size()
		button-variant(secondary, secondary-hover)

	.submit-btn
		button-base()
		button-size()
		button-variant(danger, danger-hover)
		gap spacing-sm

	.content
		display grid
		gap spacing-2xl

	.empty-state
		text-align center
		padding spacing-3xl

		h3
			margin spacing-lg 0 spacing-sm 0
			color text-primary
			font-size font-size-xl

		p
			margin 0
			color text-muted

	.blocked-users-list
		flex-column()
		gap spacing-lg

	.blocked-user-card
		card-base()
		card-padding(spacing-xl)
		border-left 4px solid danger
		box-shadow shadow-sm

	.user-header
		flex-between()
		align-items center
		margin-bottom spacing-md

		+mobile()
			flex-direction column
			align-items flex-start
			gap spacing-sm

	.user-info
		flex-center()
		gap spacing-sm
		color text-secondary

	.user-tag
		font-weight font-weight-semibold
		color text-primary

	.user-id
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

	.reason,
	.blocked-by
		margin-bottom spacing-sm
		color text-primary
		font-size font-size-base

	.actions
		display flex
		justify-content flex-end
		margin-top spacing-lg

	.unblock-btn
		button-base()
		button-size()
		button-variant(success, success-hover)
		gap spacing-sm
</style>
