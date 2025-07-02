<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let loading = $state(false);

	function handleServerSelect(guildId: string) {
		loading = true;
		const form = document.createElement('form');
		form.method = 'POST';
		form.action = '?/selectServer';

		const input = document.createElement('input');
		input.type = 'hidden';
		input.name = 'guild_id';
		input.value = guildId;

		form.appendChild(input);
		document.body.appendChild(form);
		form.submit();
	}
</script>

<svelte:head>
	<title>Select Server - Modmail Dashboard</title>
</svelte:head>

<div class="select-server-container">
	<div class="select-server-card">
		<div class="header">
			<h1>Select Server</h1>
			<p>Choose which Discord server you want to manage modmail for:</p>
		</div>

		{#if data.error}
			<div class="error-message">
				{data.error}
				<button onclick={() => goto('/login')} class="retry-btn"> Try Again </button>
			</div>
		{:else if data.guilds}
			<div class="servers-grid">
				{#each data.guilds as guild}
					<button
						class="server-card"
						onclick={() => handleServerSelect(guild.guild_id)}
						disabled={loading}
					>
						{#if guild.guild_icon}
							<img
								src={`https://cdn.discordapp.com/icons/${guild.guild_id}/${guild.guild_icon}.png`}
								alt={guild.guild_name}
								class="server-icon"
							/>
						{:else}
							<div class="server-icon-placeholder">
								{guild.guild_name.charAt(0).toUpperCase()}
							</div>
						{/if}
						<div class="server-info">
							<h3>{guild.guild_name}</h3>
							<p>Click to manage this server</p>
						</div>
					</button>
				{/each}
			</div>
		{/if}

		<div class="footer">
			<form action="/api/auth/logout" method="post">
				<button type="submit" class="logout-btn">Logout</button>
			</form>
		</div>
	</div>
</div>

<style lang="stylus">
	@import '../../styles/_variables.styl'
	@import '../../styles/_mixins.styl'

	.select-server-container
		min-height 100vh
		flex-center()
		background linear-gradient(135deg, #667eea 0%, #764ba2 100%)
		padding spacing-2xl

	.select-server-card
		background bg-white
		border-radius radius-xl
		box-shadow shadow-2xl
		max-width 600px
		width 100%
		overflow hidden

	.header
		background text-primary
		color white
		padding spacing-2xl
		text-align center

		h1
			margin 0 0 spacing-sm 0
			font-size font-size-2xl
			font-weight font-weight-semibold

		p
			margin 0
			opacity 0.9
			font-size font-size-base

	.error-message
		padding spacing-2xl
		text-align center
		color danger

		.retry-btn
			button-base()
			button-size()
			button-variant(primary, primary-hover)
			margin-top spacing-lg

	.servers-grid
		padding spacing-2xl
		display grid
		gap spacing-lg
		grid-template-columns repeat(auto-fit, minmax(250px, 1fr))

		+mobile()
			grid-template-columns 1fr

	.server-card
		button-base()
		card-base()
		card-hover()
		card-padding(spacing-xl)
		text-align left
		background bg-white
		border 2px solid border-light
		transition all transition-normal

		&:hover:not(:disabled)
			border-color primary
			transform translateY(-2px)
			box-shadow shadow-lg

		&:disabled
			opacity 0.6
			cursor not-allowed

	.server-icon
		width 48px
		height 48px
		border-radius radius-lg
		margin-bottom spacing-md

	.server-icon-placeholder
		width 48px
		height 48px
		border-radius radius-lg
		background primary
		color white
		flex-center()
		font-size font-size-xl
		font-weight font-weight-bold
		margin-bottom spacing-md

	.server-info
		h3
			margin 0 0 spacing-xs 0
			color text-primary
			font-size font-size-lg
			font-weight font-weight-semibold

		p
			margin 0
			color text-secondary
			font-size font-size-sm

	.footer
		padding spacing-xl
		border-top 1px solid border-light
		text-align center

	.logout-btn
		button-base()
		button-size()
		button-variant(secondary, secondary-hover)
</style>
