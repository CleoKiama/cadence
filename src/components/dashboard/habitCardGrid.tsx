import { Card } from "#/components/ui/card";
import { Badge } from "#/components/ui/badge";
import { CodeIcon, Flame, Minus, TrendingDown, TrendingUp } from "lucide-react";
import { MetricSummary } from "./habitCard";

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
      return "text-success";
    case "down":
      return "text-destructive";
    default:
      return "text-muted-foreground";
  }
};

export const HabitCard = (metric: MetricSummary) => {
  return (
    <Card className="relative overflow-hidden hover:shadow-lg hover:-translate-y-1 transition-all duration-200 p-6">
      <div className="flex items-start justify-between">
        <div className="flex items-center space-x-3">
          <div className="p-2 rounded-lg bg-primary text-primary-foreground">
            <CodeIcon />
          </div>
          <div>
            <h3 className="font-semibold text-lg text-foreground">
              {metric.displayName}
            </h3>
            <p className="text-sm text-muted-foreground">
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
            <span className="text-2xl font-bold text-foreground">
              {metric.currentStreak}
            </span>
          </div>
          <p className="text-sm text-muted-foreground">Current Streak</p>
        </div>

        <div className="text-center">
          <div className="text-2xl font-bold text-foreground">
            {metric.longestStreak}
          </div>
          <p className="text-sm text-muted-foreground">Best Streak</p>
        </div>
      </div>

      <div className="mt-4 pt-4 border-t border-border">
        <div className="flex justify-between items-center">
          <div>
            <p className="text-sm text-muted-foreground">Weekly Avg</p>
            <p className="font-semibold text-foreground">
              {metric.weeklyAverage}
            </p>
          </div>
          <div>
            <p className="text-sm text-muted-foreground">Monthly Total</p>
            <p className="font-semibold text-foreground">
              {metric.monthlyTotal}
            </p>
          </div>
          <div>
            <Badge variant={metric.currentStreak > 0 ? "secondary" : "default"}>
              {metric.currentStreak > 0 ? "Active" : "Inactive"}
            </Badge>
          </div>
        </div>
      </div>
    </Card>
  );
};
