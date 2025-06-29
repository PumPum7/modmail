<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { enhance } from '$app/forms';
	import type { Macro } from '$lib/api';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

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
			<form
				method="POST"
				action="?/create"
				use:enhance={() => {
					loading = true;
					clearMessages();
					return async ({ result }) => {
						loading = false;
						if (result.type === 'success') {
							await invalidateAll();
							showCreateForm = false;
							newMacroName = '';
							newMacroContent = '';
							newMacroQuickAccess = false;
							success = 'Macro created successfully!';
						}
					};
				}}
			>
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
			<form
				method="POST"
				action="?/update"
				use:enhance={() => {
					loading = true;
					clearMessages();
					return async ({ result }) => {
						loading = false;
						if (result.type === 'success') {
							await invalidateAll();
							editingMacro = null;
							editMacroContent = '';
							editMacroQuickAccess = false;
							success = 'Macro updated successfully!';
						}
					};
				}}
			>
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
							<form
								method="POST"
								action="?/delete"
								use:enhance={() => {
									loading = true;
									clearMessages();
									return async ({ result }) => {
										loading = false;
										if (result.type === 'success') {
											await invalidateAll();
											success = 'Macro deleted successfully!';
										}
									};
								}}
							>
								<input type="hidden" name="name" value={macro.name} />
								<button class="btn btn-small btn-danger" type="submit" disabled={loading}>
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

<style lang="stylus">
	@import '../../styles/_variables.styl'
	@import '../../styles/_mixins.styl'

	.container
		padding-top spacing-2xl
		padding-bottom spacing-2xl
		container()

		+mobile()
			padding spacing-lg

	header
		margin-bottom spacing-2xl

		h1
			margin 0 0 spacing-sm 0
			color text-primary

		p
			margin 0
			color text-secondary
			font-size font-size-lg

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

	.actions
		display flex
		gap spacing-lg
		margin-bottom spacing-2xl

		+mobile()
			flex-direction column

	.btn
		button-base()
		button-size(spacing-md, spacing-xl)

		&.btn-primary
			button-variant(primary, primary-hover)

		&.btn-secondary
			button-variant(secondary, secondary-hover)

		&.btn-danger
			button-variant(danger, danger-hover)

		&.btn-small
			button-size(spacing-sm, spacing-lg, font-size-sm)

	.form-card
		card-base()
		card-padding(spacing-2xl)
		margin-bottom spacing-2xl

		h3
			margin 0 0 spacing-xl 0
			color text-primary

	.form-group
		margin-bottom spacing-xl

		label
			form-label()

		input,
		textarea
			form-input()

	.form-actions
		display flex
		gap spacing-lg

		+mobile()
			flex-direction column

	.loading
		text-align center
		padding spacing-2xl
		color text-secondary
		font-size font-size-lg

	.empty-state
		text-align center
		padding spacing-3xl
		color text-secondary

		h3
			margin 0 0 spacing-lg 0
			color text-primary

	.macros-grid
		grid-auto-fill(400px, spacing-xl)

		+mobile()
			grid-template-columns 1fr

	.macro-card
		card-base()
		card-hover()
		card-padding()

	.macro-header
		flex-between()
		align-items flex-start
		margin-bottom spacing-lg

		h4
			margin 0
			color text-primary
			flex 1

		+mobile()
			flex-direction column
			gap spacing-lg

	.macro-actions
		display flex
		gap spacing-sm

		+mobile()
			align-self stretch

			.btn-small
				flex 1

	.macro-content
		color text-secondary
		line-height line-height-normal
		white-space pre-wrap
		background bg-gray
		padding spacing-lg
		border-radius radius-md
		border 1px solid border-light

	.checkbox-label
		display flex
		align-items center
		gap spacing-sm
		font-weight font-weight-normal
		cursor pointer

		input[type="checkbox"]
			width auto
			margin 0

	.quick-access-badge
		status-badge(primary, white)
		font-size font-size-xs
		margin-left spacing-sm
</style>
