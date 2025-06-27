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

			if (!data.user.isModerator) {
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
							{formatNumber(data.overview.open_threads)} open, {formatNumber(data.overview.closed_threads)} closed
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
										style="height: {(day.count / Math.max(...data.threadVolume.map(d => d.count))) * 100}%"
									></div>
									<div class="bar-label">{new Date(day.date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' })}</div>
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
							<div class="response-value">{formatHours(data.responseTimes.avg_first_response_hours)}</div>
						</div>
						<div class="response-metric">
							<div class="response-label">Median First Response</div>
							<div class="response-value">{formatHours(data.responseTimes.median_first_response_hours)}</div>
						</div>
						<div class="response-metric">
							<div class="response-label">Average Resolution Time</div>
							<div class="response-value">{formatHours(data.responseTimes.avg_resolution_time_hours)}</div>
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

<style>
	.page {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.header-left {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.back-btn {
		background: #6b7280;
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.9rem;
		align-self: flex-start;
		transition: background-color 0.2s;
	}

	.back-btn:hover {
		background: #4b5563;
	}

	.title-section h1 {
		margin: 0 0 0.5rem 0;
		color: #2c2f36;
		font-size: 1.75rem;
		font-weight: 600;
	}

	.title-section p {
		margin: 0;
		color: #666;
		font-size: 0.9rem;
	}

	.alert {
		padding: 1rem;
		border-radius: 0.5rem;
		margin-bottom: 1rem;
		background-color: #fee;
		color: #c53030;
		border: 1px solid #feb2b2;
	}

	.content {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
	}

	.metric-card {
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 1.5rem;
		display: flex;
		align-items: center;
		gap: 1rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.metric-icon {
		background: #f0f9ff;
		color: #0369a1;
		padding: 0.75rem;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.metric-content h3 {
		margin: 0 0 0.5rem 0;
		color: #374151;
		font-size: 0.9rem;
		font-weight: 500;
	}

	.metric-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #2c2f36;
		margin-bottom: 0.25rem;
	}

	.metric-detail {
		font-size: 0.8rem;
		color: #666;
	}

	.chart-section {
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 1.5rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.chart-section h2 {
		margin: 0 0 1.5rem 0;
		color: #374151;
		font-size: 1.25rem;
		font-weight: 600;
	}

	.chart-container {
		height: 200px;
		display: flex;
		align-items: flex-end;
	}

	.bar-chart {
		display: flex;
		align-items: flex-end;
		gap: 4px;
		width: 100%;
		height: 100%;
	}

	.bar-item {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		height: 100%;
		position: relative;
	}

	.bar {
		background: linear-gradient(to top, #3b82f6, #60a5fa);
		width: 100%;
		min-height: 4px;
		border-radius: 2px 2px 0 0;
		transition: all 0.2s;
	}

	.bar:hover {
		background: linear-gradient(to top, #2563eb, #3b82f6);
	}

	.bar-label {
		font-size: 0.7rem;
		color: #666;
		margin-top: 0.5rem;
		text-align: center;
	}

	.bar-value {
		position: absolute;
		top: -1.5rem;
		font-size: 0.7rem;
		color: #374151;
		font-weight: 500;
	}

	.empty-chart {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		color: #666;
		font-style: italic;
	}

	.response-times-section {
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 1.5rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.response-times-section h2 {
		margin: 0 0 1.5rem 0;
		color: #374151;
		font-size: 1.25rem;
		font-weight: 600;
	}

	.response-metrics {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.5rem;
	}

	.response-metric {
		text-align: center;
		padding: 1rem;
		background: #f8f9fa;
		border-radius: 6px;
	}

	.response-label {
		font-size: 0.9rem;
		color: #666;
		margin-bottom: 0.5rem;
	}

	.response-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #2c2f36;
	}

	.moderator-section {
		background: white;
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 1.5rem;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.moderator-section h2 {
		margin: 0 0 1.5rem 0;
		color: #374151;
		font-size: 1.25rem;
		font-weight: 600;
	}

	.moderator-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.moderator-card {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background: #f8f9fa;
		border-radius: 6px;
		border: 1px solid #e9ecef;
	}

	.moderator-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #374151;
	}

	.moderator-name {
		font-weight: 500;
	}

	.moderator-stats {
		display: flex;
		gap: 1.5rem;
	}

	.stat {
		text-align: center;
	}

	.stat-value {
		display: block;
		font-size: 1.25rem;
		font-weight: 700;
		color: #2c2f36;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #666;
	}

	.empty-state {
		text-align: center;
		padding: 2rem;
		color: #666;
		font-style: italic;
	}

	.loading {
		text-align: center;
		padding: 3rem;
		color: #666;
		font-size: 1.1rem;
	}

	@media (max-width: 768px) {
		.page {
			padding: 1rem;
		}

		.metrics-grid {
			grid-template-columns: 1fr;
		}

		.response-metrics {
			grid-template-columns: 1fr;
		}

		.moderator-card {
			flex-direction: column;
			gap: 1rem;
			align-items: stretch;
		}

		.moderator-stats {
			justify-content: space-around;
		}
	}
</style>
