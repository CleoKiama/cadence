"use client";

import { TrendingUp } from "lucide-react";
import { Bar, BarChart, CartesianGrid, XAxis, YAxis } from "recharts";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "#/components/ui/card";
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "#/components/ui/chart";
import { WeeklyActivity } from "#/utils/analyticsData.server";

const chartConfig = {
  value: {
    label: "value",
  },
  sun: {
    label: "Sun",
  },
  mon: {
    label: "Mon",
  },
  tue: {
    label: "Tue",
  },
  wed: {
    label: "Wed",
  },
  thu: {
    label: "Thu",
  },
  fri: {
    label: "Wed",
  },
  sat: {
    label: "Sat",
  },
} satisfies ChartConfig;

export function ChartBarActive({ data }: { data: WeeklyActivity }) {
  const chartData = data.map((item) => {
    const day = new Date(item.date)
      .toLocaleDateString("en-US", { weekday: "short" })
      .toLowerCase();
    return {
      day,
      value: item.value,
      fill: "var(--chart-3)",
    };
  });

  return (
    <Card>
      <CardHeader>
        <CardTitle>Weekly Activity</CardTitle>
      </CardHeader>
      <CardContent>
        <ChartContainer config={chartConfig}>
          <BarChart accessibilityLayer data={chartData}>
            <CartesianGrid vertical={false} />
            <XAxis
              dataKey="day"
              tickLine={false}
              tickMargin={10}
              axisLine={false}
              tickFormatter={(value) => {
                return chartConfig[value as keyof typeof chartConfig]?.label;
              }}
            />
            <YAxis />

            <ChartTooltip
              cursor={false}
              content={<ChartTooltipContent hideLabel />}
            />
            <Bar dataKey="value" barSize={40} strokeWidth={2} radius={8} />
          </BarChart>
        </ChartContainer>
      </CardContent>
      <CardFooter className="flex-col items-start gap-2 text-sm">
        <div className="text-muted-foreground leading-none">
          Showing total habits completed each day over the past week.
        </div>
      </CardFooter>
    </Card>
  );
}
