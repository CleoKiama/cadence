import React, { useEffect, useState } from "react";
import { MetricGrid } from "#/components/dashboard/MetricGrid";
import { StreakCounter } from "#/components/dashboard/StreakCounter";
import RecentActivity from "#/components/dashboard/recentActivity";
import { DashboardLoadingState } from "#/components/dashboard/DashboardLoadingState";
import { DashboardEmptyState } from "#/components/dashboard/DashboardEmptyState";
import { EmptyState } from "#/components/shared/EmptyState";
import { Button } from "#/components/ui/button";
import { useNavigationContext } from "#/contexts/NavigationContext";
import { formatDisplayDate } from "#/utils/dateUtils";
import { invoke } from "@tauri-apps/api/core";
import { type MetricSummary } from "#/components/dashboard/MetricGrid";
import { AlertCircle, Settings } from "lucide-react";
import { fetchHabitTrends } from "#/utils/analyticsData.server";
import z from "zod";
import { tryCatch } from "#/lib/utils";
import StreakGrid from "#/components/dashboard/streakGrid";
import getStreakData from "#/utils/streakGridData.server";
import { ChartDataSchema } from "#/utils/activityDataSchema.server";

interface DashboardProps {
	habitName: string;
}

interface DashboardState {
	hasMetrics: boolean | null;
	currentStreak: number | null;
	longestStreak: number | null;
	loading: boolean;
	error: string | null;
}

type StreakGridData = z.infer<typeof ChartDataSchema>;

export const Dashboard: React.FC<DashboardProps> = ({ habitName }) => {
	const [dashboardState, setDashboardState] = useState<DashboardState>({
		hasMetrics: null,
		currentStreak: null,
		longestStreak: null,
		loading: true,
		error: null,
	});

	const [streakGridData, setStreakGridData] = useState<StreakGridData[]>([]);

	const { navigateToSettings } = useNavigationContext();

	useEffect(() => {
		const checkAndFetchDashboardData = async () => {
			try {
				setDashboardState((prev) => ({ ...prev, loading: true, error: null }));

				// 1. Check if metrics exist first
				const metrics = await invoke<MetricSummary[] | null>(
					"get_dashboard_metrics",
				);

				if (!metrics || metrics.length === 0) {
					setDashboardState((prev) => ({
						...prev,
						hasMetrics: false,
						loading: false,
					}));
					return; // Don't fetch other data if no metrics
				}

				// 2. We have metrics, set hasMetrics to true
				// TODO: Update to a carousel with multiple habits
				const primaryHabit = metrics[0].name; // Use first metric as primary for streaks
				setDashboardState((prev) => ({
					...prev,
					hasMetrics: true,
					primaryHabitName: primaryHabit,
				}));

				// 3. Fetch streak data for the primary habit
				const [currentStreakResult, longestStreakResult] = await Promise.all([
					invoke<number | null>("get_current_streak", {
						habitName: primaryHabit,
					}),
					invoke<number | null>("get_longest_streak", {
						habitName: primaryHabit,
					}),
				]);

				setDashboardState((prev) => ({
					...prev,
					currentStreak: currentStreakResult,
					longestStreak: longestStreakResult,
					loading: false,
				}));

				//TODO: fetch data based on number of days of the current month
				const { data, error } = await tryCatch(getStreakData());
				if (error) throw error;
				setStreakGridData(data);
			} catch (error) {
				console.error("Failed to fetch dashboard data:", error);
				setDashboardState((prev) => ({
					...prev,
					error:
						error instanceof Error
							? error.message
							: "Failed to load dashboard data",
					hasMetrics: false,
					loading: false,
				}));
			}
		};

		checkAndFetchDashboardData();
	}, [habitName]);

	// Loading state
	if (dashboardState.loading) {
		return <DashboardLoadingState />;
	}

	// Error state
	if (dashboardState.error) {
		return (
			<div className="space-y-8">
				<div className="text-center py-8">
					<h1 className="text-3xl font-bold text-foreground mb-2">
						Welcome back to Habitron
					</h1>
					<p className="text-muted-foreground">
						Track your habits, build your future
					</p>
				</div>

				<EmptyState
					icon={<AlertCircle className="h-12 w-12 text-destructive mx-auto" />}
					title="Unable to Load Dashboard"
					description={`There was an error loading your dashboard data: ${dashboardState.error}`}
					action={
						<div className="space-x-3">
							<Button
								onClick={() => window.location.reload()}
								variant="outline"
							>
								Retry
							</Button>
							<Button onClick={navigateToSettings}>
								<Settings className="mr-2 h-4 w-4" />
								Go to Settings
							</Button>
						</div>
					}
				/>
			</div>
		);
	}

	// Empty state - no metrics tracked
	if (dashboardState.hasMetrics === false) {
		return <DashboardEmptyState />;
	}

	const streakGridItems = streakGridData.map((item, index) => (
		<StreakGrid key={index} data={item.data} habitName={habitName} />
	));

	// Full dashboard with data
	return (
		<div className="space-y-8">
			{/* Welcome Section */}
			<div className="text-center py-8">
				<h1 className="text-3xl font-bold text-foreground mb-2">
					Welcome back to Habitron
				</h1>
				<p className="text-muted-foreground">
					{formatDisplayDate(new Date())} • Track your habits, build your future
				</p>
			</div>
			<StreakCounter
				current={dashboardState.currentStreak}
				longest={dashboardState.longestStreak}
				metricName={habitName}
			/>

			{/* Metrics summary grid cards */}
			<MetricGrid />

			{/* streak grid */}
			{streakGridData && streakGridData.length > 0 && (
				<div className="grid grid-cols-1 md:grid-cols-2 gap-1">
					{streakGridItems}
				</div>
			)}

			<RecentActivity />
		</div>
	);
};
