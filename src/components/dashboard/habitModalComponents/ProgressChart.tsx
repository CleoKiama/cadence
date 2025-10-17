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
  const [yAxisMaxValue, setYAxisMaxvalue] = useState<number | undefined>(
    undefined,
  );
  console.log("yAxisMaxValue", yAxisMaxValue);
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
      const yValues: number[] = [];
      let data = parsedResult.data.prevWeek.map((item, i) => {
        const date = new Date(item.date);
        const day = date.toDateString().split(" ")[0];
        yValues.push(item.value);
        if (parsedResult.data.currentWeek[i]?.value !== undefined) {
          yValues.push(parsedResult.data.currentWeek[i].value);
        }
        yValues.push();
        let entry: ChartDataEntry = {
          day,
          prevWeekValue: item.value,
        };
        let currentWeekValue = parsedResult.data.currentWeek[i]?.value;
        if (currentWeekValue !== undefined)
          entry["currentWeekValue"] = currentWeekValue;
        return entry;
      });
      const maxDataValue = Math.max(...yValues);
      const bufferedMax = Math.floor(maxDataValue * 1.1); // Add 10%
      setYAxisMaxvalue(bufferedMax);
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
              left: 10,
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
            <YAxis domain={[0, yAxisMaxValue || "auto"]} />
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
            ></Line>
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
            ></Line>
          </LineChart>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}
