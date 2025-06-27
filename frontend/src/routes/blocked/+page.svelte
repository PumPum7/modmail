<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { goto } from '$app/navigation';
	import { Clock, User, Shield, Plus, Trash2 } from 'lucide-svelte';
	import type { PageProps } from './$types';
	import { formatDate } from '$lib/util';
	import { api } from '$lib/api';

	let { data }: PageProps = $props();

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

	async function blockUser() {
		if (!newBlockUserId.trim() || !newBlockUserTag.trim()) {
			error = 'User ID and User Tag are required';
			return;
		}

		try {
			loading = true;
			error = '';

			if (!data.user) {
				goto('/login?error=not_moderator');
				return;
			}

			await api.blockUser({
				user_id: newBlockUserId.trim(),
				user_tag: newBlockUserTag.trim(),
				blocked_by: data.user.id,
				blocked_by_tag: data.user.username,
				reason: newBlockReason.trim() || undefined
			});

			success = 'User blocked successfully!';
			newBlockUserId = '';
			newBlockUserTag = '';
			newBlockReason = '';
			showBlockForm = false;
			await invalidateAll();
		} catch (err) {
			error = 'Failed to block user';
			console.error('Error blocking user:', err);
		} finally {
			loading = false;
		}
	}

	async function unblockUser(userId: string, userTag: string) {
		try {
			loading = true;
			error = '';

			await api.unblockUser(userId);

			success = `User ${userTag} unblocked successfully!`;
			await invalidateAll();
		} catch (err) {
			error = 'Failed to unblock user';
			console.error('Error unblocking user:', err);
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
			<form onsubmit={blockUser} class="block-form">
				<div class="form-row">
					<div class="form-group">
						<label for="userId">User ID:</label>
						<input
							id="userId"
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
							<button
								onclick={() => unblockUser(blockedUser.user_id, blockedUser.user_tag)}
								class="unblock-btn"
								disabled={loading}
							>
								<Trash2 size={16} />
								Unblock
							</button>
						</div>
					</div>
				{/each}
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

	.title-section h1 {
		margin: 0 0 0.5rem 0;
		color: #2c2f36;
		font-size: 1.75rem;
		font-weight: 600;
	}

	.title-section p {
		margin: 0;
		color: #666;
		font-size: 0.9rem;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.block-btn {
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

	.block-btn:hover {
		background: #c73e3e;
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

	.block-form-section {
		background: #fff5f5;
		border: 1px solid #ed4245;
		border-radius: 8px;
		padding: 1.5rem;
		margin-bottom: 2rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
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

	.block-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
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
		border-color: #ed4245;
		box-shadow: 0 0 0 3px rgba(237, 66, 69, 0.1);
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
	}

	.cancel-btn {
		background: #6b7280;
		color: white;
		border: none;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: background-color 0.2s;
	}

	.cancel-btn:hover {
		background: #4b5563;
	}

	.submit-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: #ed4245;
		color: white;
		border: none;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: background-color 0.2s;
	}

	.submit-btn:hover:not(:disabled) {
		background: #c73e3e;
	}

	.submit-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.content {
		display: grid;
		gap: 2rem;
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

	.blocked-users-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.blocked-user-card {
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 1.25rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
		border-left: 4px solid #ed4245;
	}

	.user-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.user-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #666;
	}

	.user-tag {
		font-weight: 600;
		color: #2c2f36;
	}

	.user-id {
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

	.reason,
	.blocked-by {
		margin-bottom: 0.5rem;
		color: #2c2f36;
		font-size: 0.9rem;
	}

	.actions {
		display: flex;
		justify-content: flex-end;
		margin-top: 1rem;
	}

	.unblock-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: #28a745;
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: background-color 0.2s;
	}

	.unblock-btn:hover:not(:disabled) {
		background: #218838;
	}

	.unblock-btn:disabled {
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

		.form-row {
			grid-template-columns: 1fr;
		}

		.user-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
		}
	}
</style>
