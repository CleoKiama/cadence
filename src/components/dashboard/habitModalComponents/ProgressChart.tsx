import { Card, CardContent, CardHeader, CardTitle } from "#/components/ui/card";
import { CartesianGrid, LabelList, Line, LineChart, XAxis } from "recharts";
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "#/components/ui/chart";

const chartData = [
  { day: "Sun", currentWeekValue: 214, prevWeekValue: 198 },
  { day: "Mon", currentWeekValue: 186, prevWeekValue: 205 },
  { day: "Tue", currentWeekValue: 305, prevWeekValue: 278 },
  { day: "Wed", currentWeekValue: 237, prevWeekValue: 241 },
  { day: "Thu", currentWeekValue: 73, prevWeekValue: 122 },
  { day: "Fri", currentWeekValue: 209, prevWeekValue: 190 },
  { day: "Sat", currentWeekValue: 214, prevWeekValue: 230 },
];
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
  return (
    <Card>
      <CardHeader>
        <CardTitle>Weekly Progress</CardTitle>
      </CardHeader>
      <CardContent>
        <ChartContainer config={chartConfig}>
          <LineChart
            accessibilityLayer
            data={chartData}
            margin={{
              top: 20,
              left: 12,
              right: 12,
            }}
          >
            <CartesianGrid vertical={false} />
            <XAxis
              dataKey="day"
              tickLine={false}
              axisLine={false}
              tickMargin={8}
              tickFormatter={(currentWeekValue) => currentWeekValue.slice(0, 3)}
            />
            <ChartTooltip
              cursor={false}
              content={<ChartTooltipContent indicator="line" />}
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
