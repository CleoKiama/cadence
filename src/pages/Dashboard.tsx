import React, { useEffect } from "react";
import { MetricGrid } from "#/components/dashboard/MetricGrid";
import { StreakCounter } from "#/components/dashboard/StreakCounter";
import RecentActivity from "#/components/dashboard/recentActivity";
import { formatDisplayDate } from "#/utils/dateUtils";
import { invoke } from "@tauri-apps/api/core";

interface DashboardProps {
	habitName: string;
}

export const Dashboard: React.FC<DashboardProps> = ({ habitName }) => {
	const [current_streak, setCurrentStreak] = React.useState(0);
	const [longest_streak, setLongestStreak] = React.useState(0);

	useEffect(() => {
		invoke<number>("get_current_streak", {
			habitName,
		}).then((value) => {
			console.log("Promise resolved with Current Streak:", value);
			setCurrentStreak(value);
		});

		invoke<number>("get_longest_streak", {
			habitName,
		}).then((value) => {
			console.log("Promise resolved with Current Streak:", value);
			setLongestStreak(value);
		});
	}, []);

	return (
		<div className="space-y-8">
			{/* Welcome Section */}
			<div className="text-center py-8">
				<h1 className="text-3xl font-bold text-[var(--color-foreground)] mb-2">
					Welcome back to Habitron
				</h1>
				<p className="text-[var(--color-muted-foreground)]">
					{formatDisplayDate(new Date())} • Track your habits, build your future
				</p>
			</div>
			<StreakCounter
				current={current_streak}
				longest={longest_streak}
				metricName={habitName}
			/>
			<MetricGrid />
			<RecentActivity />;
		</div>
	);
};
