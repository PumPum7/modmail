<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let { data } = $props();

	onMount(() => {
		if (data.user) {
			goto('/');
		}
	});

	function handleLogin() {
		window.location.href = '/api/auth/discord';
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

			<button onclick={handleLogin} class="discord-login-btn">
				<svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
					<path
						d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028a14.09 14.09 0 0 0 1.226-1.994a.076.076 0 0 0-.041-.106a13.107 13.107 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.299 12.299 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.839 19.839 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.955-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.946 2.418-2.157 2.418Z"
					/>
				</svg>
				Continue with Discord
			</button>
		</div>
	</div>
</div>

<style lang="stylus">
	@import '../../styles/_variables.styl'
	@import '../../styles/_mixins.styl'

	.login-container
		min-height 100vh
		flex-center()
		background linear-gradient(135deg, #667eea 0%, #764ba2 100%)
		padding spacing-2xl

	.login-card
		background bg-white
		border-radius radius-xl
		box-shadow shadow-2xl
		max-width 400px
		width 100%
		overflow hidden

	.logo
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
			opacity 0.8
			font-size font-size-base

	.login-content
		padding spacing-2xl
		text-align center

		h2
			margin 0 0 spacing-lg 0
			color text-primary
			font-size font-size-xl

		p
			margin 0 0 spacing-2xl 0
			color text-secondary
			line-height line-height-normal

	.discord-login-btn
		button-base()
		button-size(spacing-md, spacing-2xl)
		button-variant(primary, primary-hover)
		gap spacing-md
		text-decoration none

		&:active
			transform translateY(1px)
</style>
