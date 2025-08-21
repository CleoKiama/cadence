import React from "react";

import { Card } from "#/components/ui/card";
import { MetricSummary } from "#/components/dashboard/MetricGrid";

interface StatsSummaryProps {
	metrics: MetricSummary[];
	timeRange: string;
}

export const StatsSummary: React.FC<StatsSummaryProps> = ({
	metrics,
	timeRange,
}) => {
	const totalActiveMetrics = metrics.filter((m) => m.currentStreak > 0).length;
	const averageStreak =
		metrics.reduce((sum, m) => sum + m.currentStreak, 0) / metrics.length;
	const longestCurrentStreak = Math.max(...metrics.map((m) => m.currentStreak));
	const totalActivities = metrics.reduce((sum, m) => sum + m.monthlyTotal, 0);

	const stats = [
		{
			label: "Active Habits",
			value: totalActiveMetrics,
			total: metrics.length,
			format: (v: number, t: number) => `${v}/${t}`,
			color: "text-accent",
		},
		{
			label: "Avg Streak",
			value: averageStreak,
			format: (v: number) => `${v.toFixed(1)} days`,
			color: "text-primary",
		},
		{
			label: "Best Streak",
			value: longestCurrentStreak,
			format: (v: number) => `${v} days`,
			color: "text-warning",
		},
		{
			label: "Total Activities",
			value: totalActivities,
			format: (v: number) => v.toString(),
			color: "text-chart-1",
		},
	];

	return (
		<Card className="p-6 space-y-6">
			<div className="mb-6">
				<h3 className="text-lg font-semibold">Statistics Overview</h3>
				<p className="text-sm text-muted-foreground">{timeRange}</p>
			</div>

			<div className="grid grid-cols-2 lg:grid-cols-4 gap-6">
				{stats.map((stat, index) => (
					<div key={index} className="text-center">
						<div className={`text-3xl font-bold ${stat.color} mb-2`}>
							{stat.format(stat.value, (stat as any).total)}
						</div>
						<div className="text-sm text-muted-foreground">{stat.label}</div>
					</div>
				))}
			</div>

			{/* Progress indicators */}
			<div className="mt-8 space-y-4">
				{metrics.map((metric) => (
					<div key={metric.name} className="space-y-2">
						<div className="flex justify-between items-center">
							<span className="text-sm font-medium">{metric.displayName}</span>
							<span className="text-sm text-muted-foreground">
								{metric.currentStreak}/{metric.longestStreak} days
							</span>
						</div>
						<div className="w-full bg-muted rounded-full h-2">
							<div
								className="h-2 rounded-full bg-gradient-to-r from-primary to-accent transition-all duration-500"
								style={{
									width: `${Math.min((metric.currentStreak / metric.longestStreak) * 100, 100)}%`,
								}}
							/>
						</div>
					</div>
				))}
			</div>
		</Card>
	);
};
