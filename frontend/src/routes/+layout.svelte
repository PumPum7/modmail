<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { LogOut, MessageSquare, Settings, Users } from 'lucide-svelte';

	export let data;

	async function logout() {
		const response = await fetch('/auth/logout', { method: 'POST' });
		if (response.ok) {
			goto('/login');
		}
	}
</script>

<div class="app">
	{#if data.user}
		<nav class="sidebar">
			<div class="sidebar-header">
				<h1>Modmail Dashboard</h1>
			</div>
			
			<div class="nav-links">
				<a href="/" class:active={$page.url.pathname === '/'}>
					<MessageSquare size={20} />
					Threads
				</a>
				<a href="/messages" class:active={$page.url.pathname === '/messages'}>
					<Users size={20} />
					All Messages
				</a>
				<a href="/macros" class:active={$page.url.pathname === '/macros'}>
					<Settings size={20} />
					Macros
				</a>
			</div>

			<div class="sidebar-footer">
				<div class="user-info">
					<img 
						src={data.user.avatar 
							? `https://cdn.discordapp.com/avatars/${data.user.id}/${data.user.avatar}.png`
							: `https://cdn.discordapp.com/embed/avatars/${parseInt(data.user.discriminator) % 5}.png`
						}
						alt={data.user.username}
						class="avatar"
					/>
					<div class="user-details">
						<div class="username">{data.user.username}</div>
					</div>
				</div>
				<button on:click={logout} class="logout-btn">
					<LogOut size={16} />
					Logout
				</button>
			</div>
		</nav>

		<main class="content">
			<slot />
		</main>
	{:else}
		<main class="content">
			<slot />
		</main>
	{/if}
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
		background-color: #f5f5f5;
	}

	.app {
		display: flex;
		height: 100vh;
	}

	.sidebar {
		width: 250px;
		background: #2c2f36;
		color: white;
		display: flex;
		flex-direction: column;
	}

	.sidebar-header {
		padding: 1rem;
		border-bottom: 1px solid #40444b;
	}

	.sidebar-header h1 {
		margin: 0;
		font-size: 1.2rem;
		font-weight: 600;
	}

	.nav-links {
		flex: 1;
		padding: 1rem 0;
	}

	.nav-links a {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		color: #b9bbbe;
		text-decoration: none;
		transition: all 0.2s;
	}

	.nav-links a:hover {
		background: #40444b;
		color: white;
	}

	.nav-links a.active {
		background: #5865f2;
		color: white;
	}

	.sidebar-footer {
		border-top: 1px solid #40444b;
		padding: 1rem;
	}

	.user-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.avatar {
		width: 32px;
		height: 32px;
		border-radius: 50%;
	}

	.user-details {
		flex: 1;
	}

	.username {
		font-weight: 600;
		font-size: 0.9rem;
	}

	.logout-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: #ed4245;
		border: none;
		color: white;
		padding: 0.5rem 1rem;
		border-radius: 4px;
		cursor: pointer;
		font-size: 0.9rem;
		width: 100%;
		transition: background-color 0.2s;
	}

	.logout-btn:hover {
		background: #c73e3e;
	}

	.content {
		flex: 1;
		overflow: auto;
		padding: 2rem;
	}

	@media (max-width: 768px) {
		.sidebar {
			width: 200px;
		}
		
		.content {
			padding: 1rem;
		}
	}
</style> 