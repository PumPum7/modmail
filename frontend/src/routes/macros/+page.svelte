<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { enhance } from '$app/forms';
	import type { Macro } from '$lib/api';
	import type { PageProps } from './$types';

	let { data, form }: PageProps = $props();

	let loading = $state(false);
	let error = $state('');
	let success = $state('');

	// Form state
	let showCreateForm = $state(false);
	let editingMacro: Macro | null = $state(null);
	let newMacroName = $state('');
	let newMacroContent = $state('');
	let newMacroQuickAccess = $state(false);
	let editMacroContent = $state('');
	let editMacroQuickAccess = $state(false);

	// Handle form results
	$effect(() => {
		if (form?.success) {
			success = form.success;
			error = '';
			// Reset form state on success
			if (success.includes('created')) {
				newMacroName = '';
				newMacroContent = '';
				newMacroQuickAccess = false;
				showCreateForm = false;
			} else if (success.includes('updated')) {
				editingMacro = null;
				editMacroContent = '';
				editMacroQuickAccess = false;
			}
		} else if (form?.error) {
			error = form.error;
			success = '';
		}
		loading = false;
	});

	function startEditing(macro: Macro) {
		editingMacro = macro;
		editMacroContent = macro.content;
		editMacroQuickAccess = macro.quick_access;
		showCreateForm = false;
		clearMessages();
	}

	function cancelEditing() {
		editingMacro = null;
		editMacroContent = '';
		editMacroQuickAccess = false;
		clearMessages();
	}

	function cancelCreate() {
		showCreateForm = false;
		newMacroName = '';
		newMacroContent = '';
		newMacroQuickAccess = false;
		clearMessages();
	}

	function clearMessages() {
		error = '';
		success = '';
	}

	function handleSubmit() {
		loading = true;
		clearMessages();
	}
</script>

<svelte:head>
	<title>Macros - ModMail</title>
</svelte:head>

<div class="container">
	<header>
		<h1>Macro Management</h1>
		<p>Create and manage reusable message templates</p>
	</header>

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

	<div class="actions">
		{#if !showCreateForm && !editingMacro}
			<button class="btn btn-primary" onclick={() => (showCreateForm = true)}>
				+ Create New Macro
			</button>
		{/if}
		<button class="btn btn-secondary" onclick={() => invalidateAll()} disabled={loading}>
			{loading ? 'Loading...' : 'Refresh'}
		</button>
	</div>

	{#if showCreateForm}
		<div class="form-card">
			<h3>Create New Macro</h3>
			<form method="POST" action="?/create" use:enhance={() => {
				loading = true;
				clearMessages();
				return async ({ result }) => {
					loading = false;
				};
			}}>
				<div class="form-group">
					<label for="macro-name">Name:</label>
					<input
						id="macro-name"
						name="name"
						type="text"
						bind:value={newMacroName}
						placeholder="Enter macro name..."
						required
					/>
				</div>
				<div class="form-group">
					<label for="macro-content">Content:</label>
					<textarea
						id="macro-content"
						name="content"
						bind:value={newMacroContent}
						placeholder="Enter macro content..."
						rows="4"
						required
					></textarea>
				</div>
				<div class="form-group">
					<label class="checkbox-label">
						<input
							type="checkbox"
							name="quick_access"
							value="true"
							bind:checked={newMacroQuickAccess}
						/>
						Quick Access (Show as button in Discord - max 3)
					</label>
				</div>
				<div class="form-actions">
					<button type="submit" class="btn btn-primary" disabled={loading}>
						{loading ? 'Creating...' : 'Create Macro'}
					</button>
					<button type="button" class="btn btn-secondary" onclick={cancelCreate}> Cancel </button>
				</div>
			</form>
		</div>
	{/if}

	{#if editingMacro}
		<div class="form-card">
			<h3>Edit Macro: {editingMacro.name}</h3>
			<form method="POST" action="?/update" use:enhance={() => {
				loading = true;
				clearMessages();
				return async ({ result }) => {
					loading = false;
				};
			}}>
				<input type="hidden" name="name" value={editingMacro.name} />
				<div class="form-group">
					<label for="edit-macro-content">Content:</label>
					<textarea
						id="edit-macro-content"
						name="content"
						bind:value={editMacroContent}
						placeholder="Enter macro content..."
						rows="4"
						required
					></textarea>
				</div>
				<div class="form-group">
					<label class="checkbox-label">
						<input
							type="checkbox"
							name="quick_access"
							value="true"
							bind:checked={editMacroQuickAccess}
						/>
						Quick Access (Show as button in Discord - max 3)
					</label>
				</div>
				<div class="form-actions">
					<button type="submit" class="btn btn-primary" disabled={loading}>
						{loading ? 'Updating...' : 'Update Macro'}
					</button>
					<button type="button" class="btn btn-secondary" onclick={cancelEditing}> Cancel </button>
				</div>
			</form>
		</div>
	{/if}

	{#if loading && data.macros.length === 0}
		<div class="loading">Loading macros...</div>
	{:else if data.macros.length === 0}
		<div class="empty-state">
			<h3>No macros found</h3>
			<p>Create your first macro to get started!</p>
		</div>
	{:else}
		<div class="macros-grid">
			{#each data.macros as macro (macro.id)}
				<div class="macro-card">
					<div class="macro-header">
						<h4>
							{macro.name}
							{#if macro.quick_access}
								<span class="quick-access-badge">Quick Access</span>
							{/if}
						</h4>
						<div class="macro-actions">
							<button
								class="btn btn-small btn-secondary"
								onclick={() => startEditing(macro)}
								disabled={loading}
							>
								Edit
							</button>
							<form method="POST" action="?/delete" use:enhance={() => {
								loading = true;
								clearMessages();
								return async ({ result }) => {
									loading = false;
								};
							}}>
								<input type="hidden" name="name" value={macro.name} />
								<button
									class="btn btn-small btn-danger"
									type="submit"
									disabled={loading}
								>
									Delete
								</button>
							</form>
						</div>
					</div>
					<div class="macro-content">
						{macro.content}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem;
	}

	header {
		margin-bottom: 2rem;
	}

	header h1 {
		margin: 0 0 0.5rem 0;
		color: #333;
	}

	header p {
		margin: 0;
		color: #666;
		font-size: 1.1rem;
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

	.actions {
		display: flex;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.btn {
		padding: 0.75rem 1.5rem;
		border: none;
		border-radius: 0.5rem;
		cursor: pointer;
		font-size: 1rem;
		transition: all 0.2s;
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-primary {
		background-color: #4f46e5;
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background-color: #4338ca;
	}

	.btn-secondary {
		background-color: #6b7280;
		color: white;
	}

	.btn-secondary:hover:not(:disabled) {
		background-color: #4b5563;
	}

	.btn-danger {
		background-color: #dc2626;
		color: white;
	}

	.btn-danger:hover:not(:disabled) {
		background-color: #b91c1c;
	}

	.btn-small {
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
	}

	.form-card {
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		padding: 2rem;
		margin-bottom: 2rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.form-card h3 {
		margin: 0 0 1.5rem 0;
		color: #333;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: block;
		margin-bottom: 0.5rem;
		font-weight: 500;
		color: #374151;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
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
		gap: 1rem;
	}

	.loading {
		text-align: center;
		padding: 2rem;
		color: #666;
		font-size: 1.1rem;
	}

	.empty-state {
		text-align: center;
		padding: 3rem;
		color: #666;
	}

	.empty-state h3 {
		margin: 0 0 1rem 0;
		color: #374151;
	}

	.macros-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
		gap: 1.5rem;
	}

	.macro-card {
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		padding: 1.5rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		transition:
			transform 0.2s,
			box-shadow 0.2s;
	}

	.macro-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

	.macro-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
	}

	.macro-header h4 {
		margin: 0;
		color: #374151;
		flex: 1;
	}

	.macro-actions {
		display: flex;
		gap: 0.5rem;
	}

	.macro-content {
		color: #666;
		line-height: 1.5;
		white-space: pre-wrap;
		background: #f9fafb;
		padding: 1rem;
		border-radius: 0.375rem;
		border: 1px solid #e5e7eb;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: normal;
		cursor: pointer;
	}

	.checkbox-label input[type="checkbox"] {
		width: auto;
		margin: 0;
	}

	.quick-access-badge {
		background: #4f46e5;
		color: white;
		font-size: 0.75rem;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		margin-left: 0.5rem;
	}

	@media (max-width: 768px) {
		.container {
			padding: 1rem;
		}

		.macros-grid {
			grid-template-columns: 1fr;
		}

		.actions {
			flex-direction: column;
		}

		.form-actions {
			flex-direction: column;
		}

		.macro-header {
			flex-direction: column;
			gap: 1rem;
		}

		.macro-actions {
			align-self: stretch;
		}

		.btn-small {
			flex: 1;
		}
	}
</style>
