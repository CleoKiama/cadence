import { useState, useEffect } from "react";
import { HabitCard } from "./habitCard";
import { EmptyState } from "#/components/shared/EmptyState";
import { Button } from "#/components/ui/button";
import { useNavigationContext } from "#/contexts/NavigationContext";
import { z } from "zod";
import { invoke } from "@tauri-apps/api/core";
import { BarChart3, Settings } from "lucide-react";
import { Skeleton } from "#/components/ui/skeleton";

const MetricSummarySchema = z.object({
  name: z.string(),
  displayName: z.string(),
  currentStreak: z.number(),
  longestStreak: z.number(),
  weeklyAverage: z.number(),
  monthlyTotal: z.number(),
  lastUpdated: z.string(),
  trend: z.enum(["up", "down", "stable"]),
});

const MetricGridSchema = z.array(MetricSummarySchema).nullable();

export type MetricSummary = z.infer<typeof MetricSummarySchema>;

export const MetricGrid = () => {
  const [metrics, setMetrics] = useState<MetricSummary[] | null>(null);
  const [loading, setLoading] = useState(true);
  const { navigateToSettings } = useNavigationContext();

  useEffect(() => {
    const fetchMetrics = async () => {
      try {
        setLoading(true);
        const result = await invoke<MetricSummary[] | null>(
          "get_dashboard_metrics",
        );
        const parseResult = MetricGridSchema.safeParse(result);
        if (!parseResult.success) {
          console.error("Failed to parse metrics:", parseResult.error);
          console.log("Response data:", result);
          setMetrics(null);
          return;
        }
        setMetrics(parseResult.data);
      } catch (error) {
        console.error("Error fetching metrics:", error);
        setMetrics(null);
      } finally {
        setLoading(false);
      }
    };

    fetchMetrics();
  }, []);

  if (loading) {
    return (
      <div>
        <h2 className="text-xl font-semibold mb-6 text-foreground">
          Your Habits
        </h2>
        <div className="flex items-center justify-center py-8">
          <Skeleton className="h-32 w-full rounded-lg" />;
        </div>
      </div>
    );
  }

  if (!metrics || metrics.length === 0) {
    return (
      <div>
        <h2 className="text-xl font-semibold mb-6 text-foreground">
          Your Habits
        </h2>
        <EmptyState
          icon={
            <BarChart3 className="h-12 w-12 text-muted-foreground mx-auto" />
          }
          title="No Habits Tracked"
          description="Start tracking your habits to see your progress and metrics here."
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
    <div>
      <h2 className="text-xl font-semibold mb-6 text-foreground ">
        Your Habits
      </h2>
      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        {metrics.map((metric, i) => (
          <HabitCard key={i} {...metric} />
        ))}
      </div>
    </div>
  );
};
