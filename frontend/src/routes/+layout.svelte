<script lang="ts">
	import { page } from '$app/state';
	import type { LayoutData } from './$types';

	let { data, children }: { data: LayoutData; children: any } = $props();

	let showMobileMenu = $state(false);

	function isActive(path: string) {
		return page.url.pathname === path;
	}
</script>

<div class="app">
	{#if data.user}
		<nav class="navbar">
			<div class="nav-container">
				<div class="nav-brand">
					<a href="/" class="brand-link">ModMail Dashboard</a>
				</div>

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
</style>
