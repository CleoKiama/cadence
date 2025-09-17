import { useState, useEffect } from "react";
import { TrendChart } from "#/components/analytics/TrendChart";
import {
	CalendarHeatmap,
	HeatmapDataPoint,
} from "#/components/analytics/CalendarHeatmap";
import { StatsSummary } from "#/components/analytics/StatsSummary";
import { EmptyState } from "#/components/shared/EmptyState";
import { Button } from "#/components/ui/button";
import { useNavigationContext } from "#/contexts/NavigationContext";
import { getDateRange } from "#/utils/dateUtils";
import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";
import { type MetricSummary } from "#/components/dashboard/MetricGrid";
import { Loader2, TrendingUp, Settings } from "lucide-react";
import {
	ChartDataSchema,
	fetchHabitTrends,
} from "#/utils/analyticsData.server";
import { tryCatch } from "#/lib/utils";

const HeatmapPointSchema = z.object({
	date: z.string(),
	count: z.number(),
	level: z.number().min(0).max(4),
});

const AnalyticsHeatmapDataSchema = z.object({
	habitName: z.string(),
	data: z.array(HeatmapPointSchema),
});

type ChartData = z.infer<typeof ChartDataSchema>;
type AnalyticsHeatmapData = z.infer<typeof AnalyticsHeatmapDataSchema>;

export const Analytics = () => {
	const [timeRange, setTimeRange] = useState<"7d" | "30d" | "90d" | "1y">(
		"30d",
	);
	const [chartData, setChartData] = useState<ChartData[] | null>(null);
	const [heatmapData, setHeatmapData] = useState<{
		[key: string]: HeatmapDataPoint[];
	}>({});
	const [metrics, setMetrics] = useState<MetricSummary[] | null>(null);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);
	const { navigateToSettings } = useNavigationContext();

	const timeRangeOptions = [
		{ value: "7d" as const, label: "7 Days", days: 7 },
		{ value: "30d" as const, label: "30 Days", days: 30 },
		{ value: "90d" as const, label: "90 Days", days: 90 },
		{ value: "1y" as const, label: "1 Year", days: 365 },
	];

	const currentRange = timeRangeOptions.find(
		(option) => option.value === timeRange,
	)!;
	const dateRange = getDateRange(currentRange.days);

	const fetchAnalyticsData = async (days: number) => {
		try {
			setLoading(true);
			setError(null);

			// Fetch dashboard metrics for summary
			const dashboardMetrics = await invoke<MetricSummary[] | null>(
				"get_dashboard_metrics",
			);
			setMetrics(dashboardMetrics);

			// If no metrics, don't proceed with other data fetching
			if (!dashboardMetrics || dashboardMetrics.length === 0) {
				setChartData(null);
				setHeatmapData({});
				return;
			}

			// Fetch all analytic trend data
			const { data, error } = await tryCatch(fetchHabitTrends(days));
			if (error) throw error;
			setChartData(data);

			// Fetch heatmap data for each habit
			const newHeatmapData: { [key: string]: HeatmapDataPoint[] } = {};
			for (const metric of dashboardMetrics) {
				try {
					const heatmapResult = await invoke<AnalyticsHeatmapData | null>(
						"get_analytics_heatmap_data",
						{
							habitName: metric.name,
							days,
						},
					);

					if (heatmapResult) {
						const validatedHeatmapData =
							AnalyticsHeatmapDataSchema.parse(heatmapResult);
						newHeatmapData[metric.name] = validatedHeatmapData.data.map(
							(point) => ({
								date: point.date,
								count: point.count,
								level: point.level as 0 | 1 | 2 | 3 | 4,
							}),
						);
					} else {
						newHeatmapData[metric.name] = [];
					}
				} catch (err) {
					console.warn(`Failed to fetch heatmap data for ${metric.name}:`, err);
					newHeatmapData[metric.name] = [];
				}
			}
			setHeatmapData(newHeatmapData);
		} catch (err) {
			console.error("Failed to fetch analytics data:", err);
			setError(
				err instanceof Error ? err.message : "Failed to fetch analytics data",
			);
		} finally {
			setLoading(false);
		}
	};

	useEffect(() => {
		fetchAnalyticsData(currentRange.days);
	}, [timeRange, currentRange.days]);

	if (loading) {
		return (
			<div className="space-y-8">
				<div className="flex justify-center items-center h-64">
					<Loader2 className="animate-spin h-8 w-8 text-foreground" />
				</div>
			</div>
		);
	}

	if (error) {
		return (
			<div className="space-y-8">
				<div className="flex justify-center items-center h-64">
					<div className="text-destructive">Error: {error}</div>
				</div>
			</div>
		);
	}

	// Show empty state if no metrics are tracked
	if (!metrics || metrics.length === 0) {
		return (
			<div className="space-y-8">
				<div className="flex justify-between items-center">
					<div>
						<h1 className="text-2xl font-bold text-foreground">Analytics</h1>
						<p className="text-muted-foreground">
							Detailed insights into your habit tracking progress
						</p>
					</div>
				</div>

				<EmptyState
					icon={
						<TrendingUp className="h-12 w-12 text-muted-foreground mx-auto" />
					}
					title="No Analytics Data"
					description="Start tracking your habits to see detailed analytics, trends, and insights about your progress."
					action={
						<Button onClick={navigateToSettings} size="lg">
							<Settings className="mr-2 h-4 w-4" />
							Add Metrics
						</Button>
					}
				/>
			</div>
		);
	}

	return (
		<div className="space-y-8">
			{/* Header */}
			<div className="flex justify-between items-center">
				<div>
					<h1 className="text-2xl font-bold text-foreground">Analytics</h1>
					<p className="text-muted-foreground">
						Detailed insights into your habit tracking progress
					</p>
				</div>

				{/* Time Range Selector */}
				<div className="flex space-x-2">
					{timeRangeOptions.map((option) => (
						<Button
							key={option.value}
							variant={timeRange === option.value ? "default" : "ghost"}
							size="sm"
							onClick={() => setTimeRange(option.value)}
						>
							{option.label}
						</Button>
					))}
				</div>
			</div>

			{/* Stats Summary */}
			<StatsSummary
				metrics={metrics}
				timeRange={`Last ${currentRange.label.toLowerCase()}`}
			/>

			{/* Trend Charts */}
			{chartData && chartData.length > 0 && (
				<div>
					<h2 className="text-xl font-semibold mb-6 text-foreground">
						Trends Over Time
					</h2>
					<div className="space-y-6">
						{chartData.map(({ habitName, data }, i) => {
							return (
								<TrendChart
									key={i}
									title={`${habitName} Trend`}
									metricName={habitName}
									data={data}
									color="var(--chart-1)"
								/>
							);
						})}
					</div>
				</div>
			)}

			{/* Activity Heatmaps */}
			{Object.keys(heatmapData).length > 0 && (
				<div>
					<h2 className="text-xl font-semibold mb-6 text-foreground">
						Activity Heatmaps
					</h2>
					<div className="space-y-6">
						{Object.entries(heatmapData).map(([metricName, data]) => {
							const metric = metrics.find((m) => m.name === metricName);

							return (
								<CalendarHeatmap
									key={metricName}
									title={`${metric?.displayName || metricName} Activity`}
									data={data}
									startDate={dateRange.start}
									endDate={dateRange.end}
								/>
							);
						})}
					</div>
				</div>
			)}
		</div>
	);
};
