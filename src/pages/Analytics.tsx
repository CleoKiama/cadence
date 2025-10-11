import { useState, useEffect } from "react";
import { HeatmapDataPoint } from "#/components/analytics/CalendarHeatmap";
import { EmptyState } from "#/components/shared/EmptyState";
import { Button } from "#/components/ui/button";
import { useNavigationContext } from "#/contexts/NavigationContext";
import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";
import { type MetricSummary } from "#/components/dashboard/habitCardGrid";
import { Loader2, TrendingUp, Settings } from "lucide-react";
import { fetchHabitTrends } from "#/utils/analyticsData.server";
import { tryCatch } from "#/lib/utils";
import { ChartDataSchema } from "#/utils/activityDataSchema.server";

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
// type AnalyticsHeatmapData = z.infer<typeof AnalyticsHeatmapDataSchema>;

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

  const fetchAnalyticsData = async (days = 30) => {
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
    fetchAnalyticsData();
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
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-foreground">Analytics</h1>
          <p className="text-muted-foreground">
            Detailed insights into your habit tracking progress
          </p>
        </div>
      </div>
    </div>
  );
};
