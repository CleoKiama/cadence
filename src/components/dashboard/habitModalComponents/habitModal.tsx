import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "#/components/ui/dialog";
import { ScrollArea } from "#/components/ui/scroll-area";
import { MetricSummary } from "../habitCardGrid";
import { Card, CardContent, CardHeader, CardTitle } from "#/components/ui/card";
import { AwardIcon, Calendar1Icon, LucideIcon, TrendingUp } from "lucide-react";
import ChartLineLabel from "./ProgressChart";

export default function HabitModal({ metric }: { metric: MetricSummary }) {
  return (
    <Dialog>
      <DialogTrigger>More Details</DialogTrigger>
      <DialogContent className="max-w-fit">
        <DialogHeader>
          <DialogTitle asChild>
            <header className="shadow-2xl rounded-2xl p-6 bg-background">
              <h1>{metric.displayName}</h1>
            </header>
          </DialogTitle>
          <DialogDescription asChild>
            <ScrollArea>
              <HabitDetails metric={metric} />
            </ScrollArea>
          </DialogDescription>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}

const HabitDetails = ({ metric }: { metric: MetricSummary }) => {
  return (
    <div className="px-4 space-y-4">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-2">
        <StatsCard
          name="Current Streak"
          value={`${metric.currentStreak} days`}
          Icon={TrendingUp}
        />
        <StatsCard
          name="Longest Streak"
          value={`${metric.longestStreak} days`}
          Icon={AwardIcon}
        />
        <StatsCard
          name="Weekly Average"
          value={`${metric.weeklyAverage}`}
          Icon={Calendar1Icon}
        />
      </div>
      <ChartLineLabel />
    </div>
  );
};

const StatsCard = ({
  name,
  value,
  Icon,
}: {
  name: string;
  value: string;
  Icon: LucideIcon;
}) => {
  return (
    <Card>
      <CardHeader className="flex flex-row gap-1">
        <Icon className="text-muted-foreground size-5" />
        <CardTitle className="text-sm ">{name}</CardTitle>
      </CardHeader>
      <CardContent className="text-center text-lg">{value}</CardContent>
    </Card>
  );
};
