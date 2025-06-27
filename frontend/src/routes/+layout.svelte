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

<style>
	:global(body) {
		font-family: 'Inter', sans-serif;
	}

	.app {
		min-height: 100vh;
		background: #f8f9fa;
	}

	.navbar {
		background: white;
		border-bottom: 1px solid #e0e0e0;
		position: sticky;
		top: 0;
		z-index: 100;
	}

	.nav-container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 2rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
		height: 4rem;
	}

	.nav-brand .brand-link {
		font-size: 1.25rem;
		font-weight: 600;
		color: #2c2f36;
		text-decoration: none;
	}

	.mobile-menu-btn {
		display: none;
		flex-direction: column;
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.5rem;
		gap: 0.25rem;
	}

	.mobile-menu-btn span {
		width: 1.5rem;
		height: 2px;
		background: #2c2f36;
		transition: all 0.3s;
	}

	.nav-menu {
		display: flex;
		align-items: center;
		gap: 2rem;
	}

	.nav-link {
		color: #666;
		text-decoration: none;
		font-weight: 500;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		transition: all 0.2s;
	}

	.nav-link:hover,
	.nav-link.active {
		color: #5865f2;
		background: #f0f0ff;
	}

	.logout-form {
		margin: 0;
	}

	.logout-btn {
		background: #ed4245;
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: background-color 0.2s;
	}

	.logout-btn:hover {
		background: #c73e3e;
	}

	.main-content {
		padding: 2rem;
	}

	.main-content.with-nav {
		padding-top: 2rem;
	}

	@media (max-width: 768px) {
		.nav-container {
			padding: 0 1rem;
		}

		.mobile-menu-btn {
			display: flex;
		}

		.nav-menu {
			position: absolute;
			top: 100%;
			left: 0;
			right: 0;
			background: white;
			border-bottom: 1px solid #e0e0e0;
			flex-direction: column;
			align-items: stretch;
			gap: 0;
			padding: 1rem;
			transform: translateY(-100%);
			opacity: 0;
			visibility: hidden;
			transition: all 0.3s;
		}

		.nav-menu.active {
			transform: translateY(0);
			opacity: 1;
			visibility: visible;
		}

		.nav-link {
			padding: 0.75rem 1rem;
			border-radius: 6px;
		}

		.main-content {
			padding: 1rem;
		}
	}
</style>
