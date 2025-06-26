<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	export let data;

	onMount(() => {
		if (data.user) {
			goto('/');
		}
	});

	function handleLogin() {
		window.location.href = '/auth/discord';
	}
</script>

<svelte:head>
	<title>Login - Modmail Dashboard</title>
</svelte:head>

<div class="login-container">
	<div class="login-card">
		<div class="logo">
			<h1>Modmail Dashboard</h1>
			<p>Moderator Access Required</p>
		</div>

		<div class="login-content">
			<h2>Sign in with Discord</h2>
			<p>You need to be a moderator to access this dashboard.</p>
			
			<button on:click={handleLogin} class="discord-login-btn">
				<svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
					<path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028a14.09 14.09 0 0 0 1.226-1.994a.076.076 0 0 0-.041-.106a13.107 13.107 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.299 12.299 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.839 19.839 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.955-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.946 2.418-2.157 2.418Z"/>
				</svg>
				Continue with Discord
			</button>
		</div>
	</div>
</div>

<style>
	.login-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		padding: 2rem;
	}

	.login-card {
		background: white;
		border-radius: 12px;
		box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
		max-width: 400px;
		width: 100%;
		overflow: hidden;
	}

	.logo {
		background: #2c2f36;
		color: white;
		padding: 2rem;
		text-align: center;
	}

	.logo h1 {
		margin: 0 0 0.5rem 0;
		font-size: 1.5rem;
		font-weight: 600;
	}

	.logo p {
		margin: 0;
		opacity: 0.8;
		font-size: 0.9rem;
	}

	.login-content {
		padding: 2rem;
		text-align: center;
	}

	.login-content h2 {
		margin: 0 0 1rem 0;
		color: #2c2f36;
		font-size: 1.25rem;
	}

	.login-content p {
		margin: 0 0 2rem 0;
		color: #666;
		line-height: 1.5;
	}

	.discord-login-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		background: #5865f2;
		color: white;
		border: none;
		padding: 0.75rem 2rem;
		border-radius: 8px;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s;
		text-decoration: none;
	}

	.discord-login-btn:hover {
		background: #4752c4;
	}

	.discord-login-btn:active {
		transform: translateY(1px);
	}
</style> 