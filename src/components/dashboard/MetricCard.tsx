import { Card } from "../shared/Card";
import { Badge } from "../shared/Badge";
import { CodeIcon, Flame, Minus, TrendingDown, TrendingUp } from "lucide-react";
import { MetricSummary } from "./MetricGrid";

const getTrendIcon = (trend: MetricSummary["trend"]) => {
	switch (trend) {
		case "up":
			return <TrendingUp />;
		case "down":
			return <TrendingDown />;
		default:
			return <Minus />;
	}
};

const getTrendColor = (trend: MetricSummary["trend"]) => {
	switch (trend) {
		case "up":
			return "text-green-600";
		case "down":
			return "text-red-600";
		default:
			return "text-gray-500";
	}
};

export const MetricCard = (metric: MetricSummary) => {
	return (
		<Card hoverable className="relative overflow-hidden">
			<div className="flex items-start justify-between">
				<div className="flex items-center space-x-3">
					<div className="p-2 rounded-lg bg-[var(--color-primary)] text-[var(--color-primary-foreground)]">
						<CodeIcon />
					</div>
					<div>
						<h3 className="font-semibold text-lg text-[var(--color-foreground)]">
							{metric.displayName}
						</h3>
						<p className="text-sm text-[var(--color-muted-foreground)]">
							Last updated: {new Date(metric.lastUpdated).toLocaleDateString()}
						</p>
					</div>
				</div>
				<div
					className={`flex items-center space-x-1 ${getTrendColor(metric.trend)}`}
				>
					{getTrendIcon(metric.trend)}
					<span className="text-sm font-medium">{metric.trend}</span>
				</div>
			</div>

			<div className="mt-6 grid grid-cols-2 gap-4">
				<div className="text-center">
					<div className="flex items-center justify-center space-x-1">
						<Flame />
						<span className="text-2xl font-bold text-[var(--color-foreground)]">
							{metric.currentStreak}
						</span>
					</div>
					<p className="text-sm text-[var(--color-muted-foreground)]">
						Current Streak
					</p>
				</div>

				<div className="text-center">
					<div className="text-2xl font-bold text-[var(--color-foreground)]">
						{metric.longestStreak}
					</div>
					<p className="text-sm text-[var(--color-muted-foreground)]">
						Best Streak
					</p>
				</div>
			</div>

			<div className="mt-4 pt-4 border-t border-[var(--color-border)]">
				<div className="flex justify-between items-center">
					<div>
						<p className="text-sm text-[var(--color-muted-foreground)]">
							Weekly Avg
						</p>
						<p className="font-semibold text-[var(--color-foreground)]">
							{metric.weeklyAverage}
						</p>
					</div>
					<div>
						<p className="text-sm text-[var(--color-muted-foreground)]">
							Monthly Total
						</p>
						<p className="font-semibold text-[var(--color-foreground)]">
							{metric.monthlyTotal}
						</p>
					</div>
					<div>
						<Badge variant={metric.currentStreak > 0 ? "success" : "default"}>
							{metric.currentStreak > 0 ? "Active" : "Inactive"}
						</Badge>
					</div>
				</div>
			</div>
		</Card>
	);
};
