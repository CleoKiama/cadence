import { Card, CardContent, CardHeader, CardTitle } from "#/components/ui/card";
import {
  CartesianGrid,
  LabelList,
  Line,
  LineChart,
  XAxis,
  YAxis,
} from "recharts";
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "#/components/ui/chart";
import { z } from "zod";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const chartDataSchema = z.object({
  currentWeek: z.array(
    z.object({
      date: z.string(),
      value: z.number().nonnegative(),
    }),
  ),
  prevWeek: z.array(
    z.object({
      date: z.string(),
      value: z.number().nonnegative(),
    }),
  ),
});

type ChartDataEntry = {
  day: string;
  prevWeekValue: number;
  currentWeekValue?: number;
};

const chartConfig = {
  currentWeekValue: {
    label: "current week-",
    color: "var(--chart-1)",
  },
  prevWeekValue: {
    label: "previous week-",
    color: "var(--chart-2)",
  },
} satisfies ChartConfig;

export default function ChartLineLabel() {
  const [data, setData] = useState<ChartDataEntry[]>([]);
  console.log("data", data);
  const [loading, setLoading] = useState(false);
  console.log("loading", loading);

  useEffect(() => {
    const fetch = async () => {
      const result = await invoke("get_weekly_metric_stats", {
        habitName: "exercise",
        weekStartsOn: "Sun", //TODO: update to be dynamic
      });
      const parsedResult = chartDataSchema.safeParse(result);
      if (!parsedResult.success)
        return console.error(`error parsing the result ${parsedResult.error}`);
      let data = parsedResult.data.prevWeek.map((item, i) => {
        const date = new Date(item.date);
        const day = date.toDateString().split(" ")[0];
        let entry: ChartDataEntry = {
          day,
          prevWeekValue: item.value,
        };
        let currentWeekValue = parsedResult.data.currentWeek[i]?.value;
        if (currentWeekValue !== undefined)
          entry["currentWeekValue"] = currentWeekValue;
        return entry;
      });
      return data;
    };

    setLoading(true);
    fetch()
      .then((data) => {
        if (data) setData(data);
      })
      .finally(() => {
        setLoading(false);
      });
  }, []);

  return (
    <Card>
      <CardHeader>
        <CardTitle>Weekly Progress</CardTitle>
      </CardHeader>
      <CardContent>
        <ChartContainer config={chartConfig}>
          <LineChart
            accessibilityLayer
            data={data}
            margin={{
              top: 15,
              left: 20,
              right: 8,
            }}
          >
            <CartesianGrid vertical={false} />
            <XAxis
              dataKey="day"
              tickLine={true}
              axisLine={false}
              tickMargin={8}
              // tickFormatter={(currentWeekValue) => currentWeekValue.slice(0, 3)}
            />

            <ChartTooltip
              cursor={true}
              content={<ChartTooltipContent indicator="dot" />}
            />
            <Line
              dataKey="prevWeekValue"
              type="natural"
              stroke="var(--color-prevWeekValue)"
              strokeWidth={2}
              dot={{
                fill: "var(--color-prevWeekValue)",
              }}
              activeDot={{
                r: 6,
              }}
            >
              <LabelList
                position="bottom"
                offset={12}
                className="fill-foreground"
                fontSize={12}
              />
            </Line>
            <Line
              dataKey="currentWeekValue"
              type="natural"
              stroke="var(--color-currentWeekValue)"
              strokeWidth={2}
              dot={{
                fill: "var(--color-currentWeekValue)",
              }}
              activeDot={{
                r: 6,
              }}
            >
              <LabelList
                position="top"
                offset={12}
                className="fill-foreground"
                fontSize={12}
              />
            </Line>
          </LineChart>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}
