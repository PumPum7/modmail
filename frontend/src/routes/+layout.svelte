<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import type { LayoutData } from './$types';

	let { data, children }: { data: LayoutData; children: any } = $props();

	let showMobileMenu = $state(false);
	let showServerDropdown = $state(false);

	function isActive(path: string) {
		return page.url.pathname === path;
	}

	function handleServerSwitch(guildId: string) {
		// Create form to switch server
		const form = document.createElement('form');
		form.method = 'POST';
		form.action = '/api/server/switch';

		const input = document.createElement('input');
		input.type = 'hidden';
		input.name = 'guild_id';
		input.value = guildId;

		form.appendChild(input);
		document.body.appendChild(form);
		form.submit();
	}
</script>

<div class="app">
	{#if data.user}
		<nav class="navbar">
			<div class="nav-container">
				<div class="nav-brand">
					<a href="/" class="brand-link">ModMail Dashboard</a>
				</div>

				<!-- Server Selector -->
				{#if data.currentGuild}
					<div class="server-selector">
						<button
							class="current-server-btn"
							onclick={() => (showServerDropdown = !showServerDropdown)}
						>
							{#if data.currentGuild.guild_icon}
								<img
									src={`https://cdn.discordapp.com/icons/${data.currentGuild.guild_id}/${data.currentGuild.guild_icon}.png`}
									alt={data.currentGuild.guild_name}
									class="server-icon"
								/>
							{:else}
								<div class="server-icon-placeholder">
									{data.currentGuild.guild_name.charAt(0).toUpperCase()}
								</div>
							{/if}
							<span class="server-name">{data.currentGuild.guild_name}</span>
							<svg class="dropdown-arrow" viewBox="0 0 20 20" fill="currentColor">
								<path
									fill-rule="evenodd"
									d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
									clip-rule="evenodd"
								/>
							</svg>
						</button>

						{#if showServerDropdown}
							<div class="server-dropdown">
								<div class="dropdown-header">
									<h3>Switch Server</h3>
								</div>
								<div class="server-list">
									{#each data.availableGuilds as guild}
										<button
											class="server-option"
											class:active={guild.guild_id === data.selectedGuildId}
											onclick={() => {
												if (guild.guild_id !== data.selectedGuildId) {
													handleServerSwitch(guild.guild_id);
												}
												showServerDropdown = false;
											}}
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
											<span class="server-name">{guild.guild_name}</span>
											{#if guild.guild_id === data.selectedGuildId}
												<svg class="check-icon" viewBox="0 0 20 20" fill="currentColor">
													<path
														fill-rule="evenodd"
														d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
														clip-rule="evenodd"
													/>
												</svg>
											{/if}
										</button>
									{/each}
								</div>
								<div class="dropdown-footer">
									<a href="/server-management" class="manage-servers-link"> Manage Servers </a>
								</div>
							</div>
						{/if}
					</div>
				{/if}

				<button
					class="mobile-menu-btn"
					onclick={() => (showMobileMenu = !showMobileMenu)}
					aria-label="Toggle menu"
				>
					<span></span>
					<span></span>
					<span></span>
				</button>

				<div class="nav-menu" class:active={showMobileMenu}>
					<a href="/" class="nav-link" class:active={isActive('/')}>Threads</a>
					<a href="/messages" class="nav-link" class:active={isActive('/messages')}>Messages</a>
					<a href="/macros" class="nav-link" class:active={isActive('/macros')}>Macros</a>
					<a href="/blocked" class="nav-link" class:active={isActive('/blocked')}>Blocked Users</a>
					<a href="/analytics" class="nav-link" class:active={isActive('/analytics')}>Analytics</a>
					<a
						href="/server-management"
						class="nav-link"
						class:active={isActive('/server-management')}>Server Settings</a
					>
					<form action="/api/auth/logout" method="post" class="logout-form">
						<button type="submit" class="logout-btn">Logout</button>
					</form>
				</div>
			</div>
		</nav>
	{/if}

	<main class="main-content" class:with-nav={data.user}>
		{@render children()}
	</main>
</div>

<!-- Click outside to close dropdown -->
{#if showServerDropdown}
	<div class="dropdown-overlay" onclick={() => (showServerDropdown = false)}></div>
{/if}

<style lang="stylus">
	@import '../styles/_variables.styl'
	@import '../styles/_mixins.styl'

	:global(body)
		font-family font-family-base

	.app
		min-height 100vh
		background bg-primary

	.navbar
		background bg-white
		border-bottom 1px solid border-light
		position sticky
		top 0
		z-index z-navbar

	.nav-container
		height 4rem
		container()
		flex-between()

	.nav-brand .brand-link
		font-size font-size-xl
		font-weight font-weight-semibold
		color text-primary
		text-decoration none

	.mobile-menu-btn
		display none
		flex-direction column
		background none
		border none
		cursor pointer
		padding spacing-sm
		gap spacing-xs

		span
			width 1.5rem
			height 2px
			background text-primary
			transition all transition-normal

		+mobile()
			display flex

	.nav-menu
		flex-center()
		gap spacing-2xl

		+mobile()
			position absolute
			top 100%
			left 0
			right 0
			background bg-white
			border-bottom 1px solid border-light
			flex-direction column
			align-items stretch
			gap 0
			padding spacing-lg
			transform translateY(-100%)
			opacity 0
			visibility hidden
			transition all transition-normal

			&.active
				transform translateY(0)
				opacity 1
				visibility visible

	.nav-link
		color text-secondary
		text-decoration none
		font-weight font-weight-medium
		padding spacing-sm spacing-lg
		border-radius radius-md
		transition all transition-fast

		&:hover,
		&.active
			color primary
			background rgba(primary, 0.1)

		+mobile()
			padding spacing-md spacing-lg
			border-radius radius-md

	.logout-form
		margin 0

	.logout-btn
		button-base()
		button-size(spacing-sm, spacing-lg)
		button-variant(danger, danger-hover)

	.main-content
		padding spacing-2xl

		&.with-nav
			padding-top spacing-2xl

		+mobile()
			padding spacing-lg

	.server-selector
		position relative
		margin 0 spacing-xl

		+mobile()
			display none

	.current-server-btn
		button-base()
		flex-center()
		gap spacing-sm
		padding spacing-sm spacing-md
		background bg-light
		border 1px solid border-light
		border-radius radius-md
		transition all transition-normal

		&:hover
			background bg-gray-100
			border-color border-gray

	.server-icon
		width 24px
		height 24px
		border-radius radius-sm

	.server-icon-placeholder
		width 24px
		height 24px
		border-radius radius-sm
		background primary
		color white
		flex-center()
		font-size font-size-xs
		font-weight font-weight-bold

	.server-name
		font-weight font-weight-medium
		color text-primary
		max-width 150px
		truncate()

	.dropdown-arrow
		width 16px
		height 16px
		color text-secondary
		transition transform transition-normal

	.server-dropdown
		position absolute
		top 100%
		left 0
		right 0
		min-width 280px
		background bg-white
		border 1px solid border-light
		border-radius radius-lg
		box-shadow shadow-lg
		z-index z-dropdown
		margin-top spacing-xs

	.dropdown-header
		padding spacing-md spacing-lg
		border-bottom 1px solid border-light

		h3
			margin 0
			font-size font-size-sm
			font-weight font-weight-semibold
			color text-secondary
			text-transform uppercase
			letter-spacing 0.05em

	.server-list
		max-height 300px
		overflow-y auto

	.server-option
		button-base()
		width 100%
		flex-between()
		gap spacing-sm
		padding spacing-md spacing-lg
		text-align left
		transition all transition-normal

		&:hover
			background bg-light

		&.active
			background rgba(primary, 0.1)
			color primary

	.check-icon
		width 16px
		height 16px
		color success

	.dropdown-footer
		padding spacing-md spacing-lg
		border-top 1px solid border-light

	.manage-servers-link
		color primary
		text-decoration none
		font-size font-size-sm
		font-weight font-weight-medium

		&:hover
			text-decoration underline

	.dropdown-overlay
		position fixed
		top 0
		left 0
		right 0
		bottom 0
		z-index z-overlay
</style>
