<script lang="ts">
	import { goto } from '$app/navigation';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	function formatNumber(num: number): string {
		return new Intl.NumberFormat().format(num);
	}

	function formatResponseTime(hours: number | null): string {
		if (hours === null) return 'N/A';
		if (hours < 1) return `${Math.round(hours * 60)}m`;
		if (hours < 24) return `${Math.round(hours)}h`;
		return `${Math.round(hours / 24)}d`;
	}
</script>

<svelte:head>
	<title>Server Management - Modmail Dashboard</title>
</svelte:head>

<div class="server-management">
	<div class="page-header">
		<h1>Server Management</h1>
		<p>Manage your Discord server's modmail configuration and view server statistics.</p>
	</div>

	{#if data.error}
		<div class="error-message">
			{data.error}
		</div>
	{:else if data.overview}
		<!-- Server Overview Stats -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon threads">
					<svg viewBox="0 0 24 24" fill="currentColor">
						<path
							d="M20 2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h4v3c0 .6.4 1 1 1h.5c.2 0 .5-.1.7-.3L14.5 18H20c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2z"
						/>
					</svg>
				</div>
				<div class="stat-content">
					<h3>Total Threads</h3>
					<p class="stat-number">{formatNumber(data.overview.total_threads)}</p>
					<p class="stat-detail">{formatNumber(data.overview.open_threads)} open</p>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon messages">
					<svg viewBox="0 0 24 24" fill="currentColor">
						<path d="M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2z" />
					</svg>
				</div>
				<div class="stat-content">
					<h3>Total Messages</h3>
					<p class="stat-number">{formatNumber(data.overview.total_messages)}</p>
					<p class="stat-detail">Across all threads</p>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon response-time">
					<svg viewBox="0 0 24 24" fill="currentColor">
						<path
							d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm4.2 14.2L11 13V7h1.5v5.2l4.5 2.7-.8 1.3z"
						/>
					</svg>
				</div>
				<div class="stat-content">
					<h3>Avg Response Time</h3>
					<p class="stat-number">{formatResponseTime(data.overview.avg_response_time_hours)}</p>
					<p class="stat-detail">First response</p>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon blocked">
					<svg viewBox="0 0 24 24" fill="currentColor">
						<path
							d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM4 12c0-4.42 3.58-8 8-8 1.85 0 3.55.63 4.9 1.69L5.69 16.9C4.63 15.55 4 13.85 4 12zm8 8c-1.85 0-3.55-.63-4.9-1.69L18.31 7.1C19.37 8.45 20 10.15 20 12c0 4.42-3.58 8-8 8z"
						/>
					</svg>
				</div>
				<div class="stat-content">
					<h3>Blocked Users</h3>
					<p class="stat-number">{formatNumber(data.overview.blocked_users)}</p>
					<p class="stat-detail">Currently blocked</p>
				</div>
			</div>
		</div>

		<!-- Management Sections -->
		<div class="management-grid">
			<div class="management-card">
				<div class="card-header">
					<h2>Quick Actions</h2>
					<p>Common modmail management tasks</p>
				</div>
				<div class="action-buttons">
					<button onclick={() => goto('/macros')} class="action-btn primary">
						<svg viewBox="0 0 24 24" fill="currentColor">
							<path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
						Manage Macros
					</button>
					<button onclick={() => goto('/blocked')} class="action-btn secondary">
						<svg viewBox="0 0 24 24" fill="currentColor">
							<path
								d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM4 12c0-4.42 3.58-8 8-8 1.85 0 3.55.63 4.9 1.69L5.69 16.9C4.63 15.55 4 13.85 4 12zm8 8c-1.85 0-3.55-.63-4.9-1.69L18.31 7.1C19.37 8.45 20 10.15 20 12c0 4.42-3.58 8-8 8z"
							/>
						</svg>
						Manage Blocked Users
					</button>
					<button onclick={() => goto('/analytics')} class="action-btn tertiary">
						<svg viewBox="0 0 24 24" fill="currentColor">
							<path d="M3 3v18h18v-2H5V3H3zm4 14h2v-6H7v6zm4 0h2V9h-2v8zm4 0h2v-4h-2v4z" />
						</svg>
						View Analytics
					</button>
				</div>
			</div>

			<div class="management-card">
				<div class="card-header">
					<h2>Server Information</h2>
					<p>Current server details and activity</p>
				</div>
				<div class="server-info">
					<div class="info-row">
						<span class="info-label">Server ID:</span>
						<span class="info-value">{data.selectedGuildId}</span>
					</div>
					<div class="info-row">
						<span class="info-label">Threads Today:</span>
						<span class="info-value">{formatNumber(data.overview.threads_today)}</span>
					</div>
					<div class="info-row">
						<span class="info-label">Threads This Week:</span>
						<span class="info-value">{formatNumber(data.overview.threads_this_week)}</span>
					</div>
					<div class="info-row">
						<span class="info-label">Threads This Month:</span>
						<span class="info-value">{formatNumber(data.overview.threads_this_month)}</span>
					</div>
				</div>
			</div>

			<div class="management-card">
				<div class="card-header">
					<h2>Server Settings</h2>
					<p>Configure modmail behavior for this server</p>
				</div>
				<div class="settings-info">
					<p class="coming-soon">⚙️ Advanced server configuration options coming soon!</p>
					<p class="settings-note">
						For now, use Discord bot commands to configure server settings:
					</p>
					<ul class="command-list">
						<li><code>/config</code> - View current configuration</li>
						<li><code>/config set</code> - Update settings</li>
					</ul>
				</div>
			</div>
		</div>
	{/if}
</div>

<style lang="stylus">
	@import '../../styles/_variables.styl'
	@import '../../styles/_mixins.styl'

	.server-management
		container()
		padding spacing-2xl 0

	.page-header
		margin-bottom spacing-2xl

		h1
			margin 0 0 spacing-sm 0
			color text-primary
			font-size font-size-3xl
			font-weight font-weight-bold

		p
			margin 0
			color text-secondary
			font-size font-size-lg

	.error-message
		card-base()
		card-padding()
		background danger-light
		color danger
		text-align center

	.stats-grid
		display grid
		grid-template-columns repeat(auto-fit, minmax(250px, 1fr))
		gap spacing-lg
		margin-bottom spacing-2xl

	.stat-card
		card-base()
		card-padding()
		flex-center()
		gap spacing-lg

	.stat-icon
		width 48px
		height 48px
		border-radius radius-lg
		flex-center()

		svg
			width 24px
			height 24px

		&.threads
			background rgba(primary, 0.1)
			color primary

		&.messages
			background rgba(success, 0.1)
			color success

		&.response-time
			background rgba(warning, 0.1)
			color warning

		&.blocked
			background rgba(danger, 0.1)
			color danger

	.stat-content
		flex 1

		h3
			margin 0 0 spacing-xs 0
			color text-secondary
			font-size font-size-sm
			font-weight font-weight-medium
			text-transform uppercase
			letter-spacing 0.05em

		.stat-number
			margin 0 0 spacing-xs 0
			color text-primary
			font-size font-size-2xl
			font-weight font-weight-bold

		.stat-detail
			margin 0
			color text-secondary
			font-size font-size-sm

	.management-grid
		display grid
		grid-template-columns repeat(auto-fit, minmax(350px, 1fr))
		gap spacing-xl

		+mobile()
			grid-template-columns 1fr

	.management-card
		card-base()

	.card-header
		padding spacing-xl spacing-xl spacing-lg spacing-xl
		border-bottom 1px solid border-light

		h2
			margin 0 0 spacing-xs 0
			color text-primary
			font-size font-size-xl
			font-weight font-weight-semibold

		p
			margin 0
			color text-secondary
			font-size font-size-sm

	.action-buttons
		padding spacing-xl
		display flex
		flex-direction column
		gap spacing-md

	.action-btn
		button-base()
		flex-center()
		gap spacing-sm
		padding spacing-md spacing-lg
		text-align left
		border-radius radius-md
		transition all transition-normal

		svg
			width 20px
			height 20px

		&.primary
			button-variant(primary, primary-hover)

		&.secondary
			button-variant(secondary, secondary-hover)

		&.tertiary
			background bg-light
			color text-primary
			border 1px solid border-light

			&:hover
				background bg-gray-100
				border-color border-gray

	.server-info
		padding spacing-xl

	.info-row
		flex-between()
		padding spacing-sm 0
		border-bottom 1px solid border-light

		&:last-child
			border-bottom none

	.info-label
		color text-secondary
		font-weight font-weight-medium

	.info-value
		color text-primary
		font-weight font-weight-semibold

	.settings-info
		padding spacing-xl

	.coming-soon
		margin 0 0 spacing-lg 0
		padding spacing-md
		background bg-light
		border-radius radius-md
		text-align center
		color text-secondary
		font-style italic

	.settings-note
		margin 0 0 spacing-md 0
		color text-secondary

	.command-list
		margin 0
		padding-left spacing-lg

		li
			margin-bottom spacing-xs
			color text-secondary

			code
				background bg-light
				padding spacing-xs spacing-sm
				border-radius radius-sm
				font-family font-family-mono
				font-size font-size-sm
				color text-primary
</style>
