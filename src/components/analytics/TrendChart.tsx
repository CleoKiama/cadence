import React from "react";
import {
	LineChart,
	Line,
	XAxis,
	YAxis,
	CartesianGrid,
	Tooltip,
	ResponsiveContainer,
} from "recharts";
import { Card } from "../shared/Card";

interface ChartDataPoint {
	date: string;
	value: number;
	label?: string;
}

interface TrendChartProps {
	data: ChartDataPoint[];
	title: string;
	metricName: string;
	color?: string;
}

export const TrendChart: React.FC<TrendChartProps> = ({
	data,
	title,
	metricName,
	color = "var(--color-chart-1)",
}) => {
	if (data.length === 0) {
		return (
			<Card>
				<h3 className="text-lg font-semibold mb-4">{title}</h3>
				<div className="h-64 flex items-center justify-center text-[var(--color-muted-foreground)]">
					No data available
				</div>
			</Card>
		);
	}

	// Calculate recent trend
	const recent = data.slice(-7);
	const trend =
		recent.length > 1 ? recent[recent.length - 1].value - recent[0].value : 0;

	// Custom tooltip component
	const CustomTooltip = ({ active, payload, label }: any) => {
		if (active && payload && payload.length) {
			return (
				<div className="bg-[var(--color-card)] border border-[var(--color-border)] rounded-lg p-2 shadow-lg">
					<p className="text-sm font-medium">{`Date: ${label}`}</p>
					<p className="text-sm" style={{ color: payload[0].color }}>
						{`${metricName}: ${payload[0].value}`}
					</p>
				</div>
			);
		}
		return null;
	};

	return (
		<Card>
			<div className="flex justify-between items-center mb-6">
				<h3 className="text-lg font-semibold">{title}</h3>
				<div className="flex items-center space-x-4 text-sm">
					<div className="text-[var(--color-muted-foreground)]">
						7-day trend:
						<span
							className={`ml-1 font-medium ${
								trend > 0
									? "text-green-600"
									: trend < 0
										? "text-red-600"
										: "text-gray-600"
							}`}
						>
							{trend > 0 ? "+" : ""}
							{trend.toFixed(1)}
						</span>
					</div>
				</div>
			</div>

			<div className="mb-6" style={{ height: "300px" }}>
				<ResponsiveContainer width="100%" height="100%">
					<LineChart
						data={data}
						margin={{
							top: 5,
							right: 30,
							left: 20,
							bottom: 5,
						}}
					>
						<CartesianGrid
							strokeDasharray="3 3"
							stroke="var(--color-border)"
							opacity={0.3}
						/>
						<XAxis
							dataKey="date"
							stroke="var(--color-muted-foreground)"
							fontSize={12}
							tickFormatter={(value) => {
								const date = new Date(value);
								return `${date.getMonth() + 1}/${date.getDate()}`;
							}}
						/>
						<YAxis stroke="var(--color-muted-foreground)" fontSize={12} />
						<Tooltip content={<CustomTooltip />} />
						<Line
							type="monotone"
							dataKey="value"
							stroke={color}
							strokeWidth={3}
							dot={{
								fill: color,
								strokeWidth: 2,
								r: 4,
							}}
							activeDot={{
								r: 6,
								stroke: color,
								strokeWidth: 2,
								fill: "var(--color-background)",
							}}
						/>
					</LineChart>
				</ResponsiveContainer>
			</div>

			<div className="grid grid-cols-3 gap-4 text-center">
				<div>
					<div className="text-2xl font-bold text-[var(--color-foreground)]">
						{data[data.length - 1]?.value || 0}
					</div>
					<div className="text-sm text-[var(--color-muted-foreground)]">
						Latest
					</div>
				</div>
				<div>
					<div className="text-2xl font-bold text-[var(--color-foreground)]">
						{(data.reduce((sum, d) => sum + d.value, 0) / data.length).toFixed(
							1,
						)}
					</div>
					<div className="text-sm text-[var(--color-muted-foreground)]">
						Average
					</div>
				</div>
				<div>
					<div className="text-2xl font-bold text-[var(--color-foreground)]">
						{Math.max(...data.map((d) => d.value))}
					</div>
					<div className="text-sm text-[var(--color-muted-foreground)]">
						Peak
					</div>
				</div>
			</div>
		</Card>
	);
};

