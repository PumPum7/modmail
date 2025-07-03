<script lang="ts">
	import { goto } from '$app/navigation';
	import { Clock, MessageSquare, Users, TrendingUp, Activity } from 'lucide-svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	// Handle authentication
	$effect(() => {
		if (typeof window !== 'undefined') {
			if (!data.user) {
				goto('/login');
				return;
			}

			if (!data.user?.isModerator) {
				goto('/login?error=not_moderator');
				return;
			}
		}
	});

	function formatHours(hours: number | null): string {
		if (hours === null) return 'N/A';
		if (hours < 1) return `${Math.round(hours * 60)}m`;
		if (hours < 24) return `${hours.toFixed(1)}h`;
		return `${(hours / 24).toFixed(1)}d`;
	}

	function formatNumber(num: number): string {
		return new Intl.NumberFormat().format(num);
	}
</script>

<svelte:head>
	<title>Analytics - ModMail</title>
</svelte:head>

<div class="page">
	<div class="page-header">
		<div class="header-left">
			<button onclick={() => goto('/')} class="back-btn">← Back to Threads</button>
			<div class="title-section">
				<h1>Analytics Dashboard</h1>
				<p>Insights and metrics for your modmail system</p>
			</div>
		</div>
	</div>

	{#if data.error}
		<div class="alert alert-error">
			{data.error}
		</div>
	{:else if data.overview}
		<div class="content">
			<!-- Overview Cards -->
			<div class="metrics-grid">
				<div class="metric-card">
					<div class="metric-icon">
						<MessageSquare size={24} />
					</div>
					<div class="metric-content">
						<h3>Total Threads</h3>
						<div class="metric-value">{formatNumber(data.overview.total_threads)}</div>
						<div class="metric-detail">
							{formatNumber(data.overview.open_threads)} open, {formatNumber(
								data.overview.closed_threads
							)} closed
						</div>
					</div>
				</div>

				<div class="metric-card">
					<div class="metric-icon">
						<Clock size={24} />
					</div>
					<div class="metric-content">
						<h3>Avg Response Time</h3>
						<div class="metric-value">{formatHours(data.overview.avg_response_time_hours)}</div>
						<div class="metric-detail">Average first response</div>
					</div>
				</div>

				<div class="metric-card">
					<div class="metric-icon">
						<TrendingUp size={24} />
					</div>
					<div class="metric-content">
						<h3>Recent Activity</h3>
						<div class="metric-value">{formatNumber(data.overview.threads_today)}</div>
						<div class="metric-detail">
							{formatNumber(data.overview.threads_this_week)} this week
						</div>
					</div>
				</div>

				<div class="metric-card">
					<div class="metric-icon">
						<Users size={24} />
					</div>
					<div class="metric-content">
						<h3>Messages & Notes</h3>
						<div class="metric-value">{formatNumber(data.overview.total_messages)}</div>
						<div class="metric-detail">
							{formatNumber(data.overview.total_notes)} internal notes
						</div>
					</div>
				</div>
			</div>

			<!-- Thread Volume Chart -->
			<div class="chart-section">
				<h2>Thread Volume (Last 30 Days)</h2>
				<div class="chart-container">
					{#if data.threadVolume.length > 0}
						<div class="bar-chart">
							{#each data.threadVolume as day}
								<div class="bar-item">
									<div
										class="bar"
										style="height: {(day.count /
											Math.max(...data.threadVolume.map((d) => d.count))) *
											100}%"
									></div>
									<div class="bar-label">
										{new Date(day.date).toLocaleDateString('en-US', {
											month: 'short',
											day: 'numeric'
										})}
									</div>
									<div class="bar-value">{day.count}</div>
								</div>
							{/each}
						</div>
					{:else}
						<div class="empty-chart">No thread volume data available</div>
					{/if}
				</div>
			</div>

			<!-- Response Time Metrics -->
			{#if data.responseTimes}
				<div class="response-times-section">
					<h2>Response Time Metrics</h2>
					<div class="response-metrics">
						<div class="response-metric">
							<div class="response-label">Average First Response</div>
							<div class="response-value">
								{formatHours(data.responseTimes.avg_first_response_hours)}
							</div>
						</div>
						<div class="response-metric">
							<div class="response-label">Median First Response</div>
							<div class="response-value">
								{formatHours(data.responseTimes.median_first_response_hours)}
							</div>
						</div>
						<div class="response-metric">
							<div class="response-label">Average Resolution Time</div>
							<div class="response-value">
								{formatHours(data.responseTimes.avg_resolution_time_hours)}
							</div>
						</div>
					</div>
				</div>
			{/if}

			<!-- Moderator Activity -->
			<div class="moderator-section">
				<h2>Moderator Activity (Last 30 Days)</h2>
				{#if data.moderatorActivity.length > 0}
					<div class="moderator-list">
						{#each data.moderatorActivity as moderator}
							<div class="moderator-card">
								<div class="moderator-info">
									<Activity size={16} />
									<span class="moderator-name">{moderator.moderator_tag}</span>
								</div>
								<div class="moderator-stats">
									<div class="stat">
										<span class="stat-value">{formatNumber(moderator.message_count)}</span>
										<span class="stat-label">Messages</span>
									</div>
									<div class="stat">
										<span class="stat-value">{formatNumber(moderator.note_count)}</span>
										<span class="stat-label">Notes</span>
									</div>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div class="empty-state">No moderator activity data available</div>
				{/if}
			</div>
		</div>
	{:else}
		<div class="loading">Loading analytics...</div>
	{/if}
</div>

<style lang="stylus">
	@import '../../styles/_variables.styl'
	@import '../../styles/_mixins.styl'

	.page
		container(1200px)
		
		& 
			padding-top spacing-2xl
			padding-bottom spacing-2xl

		+mobile()
			padding spacing-lg

	.page-header
		margin-bottom spacing-2xl

	.header-left
		flex-column()
		gap spacing-lg

	.back-btn
		button-base()
		button-size(spacing-sm, spacing-lg, font-size-base)
		button-variant(secondary, secondary-hover)
		align-self flex-start

	.title-section
		h1
			margin 0 0 spacing-sm 0
			color text-primary
			font-size font-size-3xl
			font-weight font-weight-semibold

		p
			margin 0
			color text-secondary
			font-size font-size-base

	.alert
		alert-base()

		&.alert-error
			alert-variant(#fee, danger, #feb2b2)

	.content
		flex-column()
		gap spacing-2xl

	.metrics-grid
		grid-auto-fit(250px, spacing-xl)

		+mobile()
			grid-template-columns 1fr

	.metric-card
		card-base()
		card-padding(spacing-xl)
		flex-center()
		gap spacing-lg

	.metric-icon
		background lighten(primary, 45%)
		color darken(primary, 10%)
		padding spacing-md
		border-radius radius-lg
		flex-center()

	.metric-content
		h3
			margin 0 0 spacing-sm 0
			color text-primary
			font-size font-size-base
			font-weight font-weight-medium

	.metric-value
		font-size font-size-3xl
		font-weight font-weight-bold
		color text-primary
		margin-bottom spacing-xs

	.metric-detail
		font-size font-size-sm
		color text-secondary

	.chart-section,
	.response-times-section,
	.moderator-section
		card-base()
		card-padding(spacing-xl)

		h2
			margin 0 0 spacing-xl 0
			color text-primary
			font-size font-size-xl
			font-weight font-weight-semibold

	.chart-container
		height 200px
		display flex
		align-items flex-end

	.bar-chart
		display flex
		align-items flex-end
		gap 4px
		width 100%
		height 100%

	.bar-item
		flex 1
		flex-column()
		align-items center
		height 100%
		position relative

	.bar
		background linear-gradient(to top, primary, lighten(primary, 20%))
		width 100%
		min-height 4px
		border-radius radius-sm radius-sm 0 0
		transition all transition-fast

		&:hover
			background linear-gradient(to top, darken(primary, 10%), primary)

	.bar-label
		font-size font-size-xs
		color text-secondary
		margin-top spacing-sm
		text-align center

	.bar-value
		position absolute
		top -1.5rem
		font-size font-size-xs
		color text-primary
		font-weight font-weight-medium

	.empty-chart
		flex-center()
		width 100%
		color text-secondary
		font-style italic

	.response-metrics
		grid-auto-fit(200px, spacing-xl)

		+mobile()
			grid-template-columns 1fr

	.response-metric
		text-align center
		padding spacing-lg
		background bg-light
		border-radius radius-md

	.response-label
		font-size font-size-base
		color text-secondary
		margin-bottom spacing-sm

	.response-value
		font-size font-size-2xl
		font-weight font-weight-bold
		color text-primary

	.moderator-list
		flex-column()
		gap spacing-lg

	.moderator-card
		flex-between()
		align-items center
		padding spacing-lg
		background bg-light
		border-radius radius-md
		border 1px solid border-light

		+mobile()
			flex-direction column
			gap spacing-lg
			align-items stretch

	.moderator-info
		flex-center()
		gap spacing-sm
		color text-primary

	.moderator-name
		font-weight font-weight-medium

	.moderator-stats
		display flex
		gap spacing-xl

		+mobile()
			justify-content space-around

	.stat
		text-align center

	.stat-value
		display block
		font-size font-size-xl
		font-weight font-weight-bold
		color text-primary

	.stat-label
		font-size font-size-sm
		color text-secondary

	.empty-state
		text-align center
		padding spacing-2xl
		color text-secondary
		font-style italic

	.loading
		text-align center
		padding spacing-3xl
		color text-secondary
		font-size font-size-lg
</style>
