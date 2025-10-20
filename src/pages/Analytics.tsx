import { useState, useEffect } from "react";
import { EmptyState } from "#/components/shared/EmptyState";
import { Button } from "#/components/ui/button";
import { useNavigationContext } from "#/contexts/NavigationContext";
import { invoke } from "@tauri-apps/api/core";
import { type MetricSummary } from "#/components/dashboard/habitCardGrid";
import {
  Loader2,
  TrendingUp,
  Settings,
  TrophyIcon,
  TrendingUpIcon,
  Calendar1Icon,
  TargetIcon,
  CalendarDays,
} from "lucide-react";
import {
  AnalyticsSummary,
  getAnalyticsSummary,
  getWeeklyAcitivity,
  WeeklyActivity,
} from "#/utils/analyticsData.server";
import { tryCatch } from "#/utils/misc";
import { Card } from "#/components/ui/card";
import { ChartBarActive } from "#/components/analytics/weekly-activity-bar-active";

export const Analytics = () => {
  const [metrics, setMetrics] = useState<MetricSummary[] | null>(null);
  const [analyticsSummary, setAnalyticsSummary] = useState<AnalyticsSummary>();
  const [weeklyActivity, setWeeklyActivity] = useState<WeeklyActivity>();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { navigateToSettings } = useNavigationContext();

  const fetchAnalyticsData = async () => {
    setLoading(true);
    setError(null);

    // Fetch dashboard metrics for summary
    const dashboardMetrics = await invoke<MetricSummary[] | null>(
      "get_dashboard_metrics",
    );
    setMetrics(dashboardMetrics);

    // If no metrics, don't proceed with other data fetching
    if (!dashboardMetrics || dashboardMetrics.length === 0) {
      return;
    }

    const analiticsSummaryRes = getAnalyticsSummary();
    const { data, error } = await tryCatch(analiticsSummaryRes);
    if (error) {
      console.error("Error fetching analytics summary:", error);
      setError("failed to fetch analytics summary");
      return;
    }
    const weeklyDataRes = getWeeklyAcitivity();
    const { data: weeklyActivityData, error: weeklyDataError } =
      await tryCatch(weeklyDataRes);
    if (weeklyDataError) {
      console.error("Error fetching analytics summary:", error);
      setError("failed to fetch weekly analytics data");
      return;
    }

    setWeeklyActivity(weeklyActivityData);
    setAnalyticsSummary(data);
    setLoading(false);
  };

  useEffect(() => {
    void fetchAnalyticsData();
  }, []);

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
      <div>
        <h1 className="text-2xl font-bold text-foreground">Analytics</h1>
        <p className="text-muted-foreground">
          Detailed insights into your habit tracking progress
        </p>
      </div>
      <div className="grid sm:grid-cols-2 md:grid-cols-4 wrap-normal gap-2 ">
        {analyticsSummary &&
          Object.entries(analyticsSummary).map(([name, value], i) => {
            return <SummaryCard key={i} name={name} value={value} />;
          })}
      </div>
      <div>{weeklyActivity && <ChartBarActive data={weeklyActivity} />}</div>
    </div>
  );
};

const SummaryCard = ({ name, value }: { name: string; value: number }) => {
  const icons = {
    longestStreak: (
      <div className="w-12 h-12 flex items-center justify-center rounded-xl bg-yellow-400/10 text-yellow-400">
        <TrophyIcon className="w-6 h-6" />
      </div>
    ),
    totalHabits: (
      <div className="w-12 h-12 flex items-center justify-center rounded-xl bg-cyan-400/10 text-cyan-400">
        <TargetIcon className="w-6 h-6" />
      </div>
    ),
    completionRate: (
      <div className="w-12 h-12 flex items-center justify-center rounded-xl bg-emerald-500/10 text-emerald-500">
        <TrendingUpIcon className="w-6 h-6" />
      </div>
    ),
    activeDays: (
      <div className="w-12 h-12 flex items-center justify-center rounded-xl bg-purple-400/10 text-purple-400">
        <CalendarDays className="w-6 h-6" />
      </div>
    ),
  };
  const displayNames = {
    completionRate: "Compeletion rate",
    longestStreak: "Longest streak",
    activeDays: "Active Days",
    totalHabits: "Total Habits",
  };

  const icon = icons[name as keyof AnalyticsSummary];
  const displayName = displayNames[name as keyof AnalyticsSummary];

  return (
    <Card className="gap-2">
      {icon}
      <h3 className="text-foreground">
        {value} {displayName === "Longest streak" && "days"}
      </h3>
      <p className="text-muted-foreground  text-sm">{displayName}</p>
    </Card>
  );
};
