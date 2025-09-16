import React, { useRef, useEffect, useState } from "react";
import * as d3 from "d3";
import { Card } from "#/components/ui/card";

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
	color = "var(--chart-1)",
}) => {
	const svgRef = useRef<SVGSVGElement>(null);
	const [tooltip, setTooltip] = useState<{
		visible: boolean;
		x: number;
		y: number;
		date: string;
		value: number;
	}>({ visible: false, x: 0, y: 0, date: "", value: 0 });

	useEffect(() => {
		if (!svgRef.current || data.length === 0) return;

		const svg = d3.select(svgRef.current);
		svg.selectAll("*").remove();

		const containerRect = svgRef.current.getBoundingClientRect();
		const margin = { top: 20, right: 30, bottom: 40, left: 40 };
		const width = containerRect.width - margin.left - margin.right;
		const height = 300 - margin.top - margin.bottom;

		// Parse dates and create scales
		const parsedData = data.map(d => ({
			...d,
			date: new Date(d.date)
		}));

		const xScale = d3.scaleTime()
			.domain(d3.extent(parsedData, d => d.date) as [Date, Date])
			.range([0, width]);

		const yScale = d3.scaleLinear()
			.domain(d3.extent(parsedData, d => d.value) as [number, number])
			.nice()
			.range([height, 0]);

		// Create main group
		const g = svg.append("g")
			.attr("transform", `translate(${margin.left},${margin.top})`);

		// Add grid
		g.append("g")
			.attr("class", "grid")
			.attr("transform", `translate(0,${height})`)
			.call(d3.axisBottom(xScale)
				.tickSize(-height)
				.tickFormat(() => "")
			)
			.selectAll("line")
			.attr("stroke", "var(--border)")
			.attr("stroke-dasharray", "3,3")
			.attr("opacity", 0.3);

		g.append("g")
			.attr("class", "grid")
			.call(d3.axisLeft(yScale)
				.tickSize(-width)
				.tickFormat(() => "")
			)
			.selectAll("line")
			.attr("stroke", "var(--border)")
			.attr("stroke-dasharray", "3,3")
			.attr("opacity", 0.3);

		// Add axes
		g.append("g")
			.attr("transform", `translate(0,${height})`)
			.call(d3.axisBottom(xScale)
				.tickFormat((d) => {
					const date = d as Date;
					return `${date.getMonth() + 1}/${date.getDate()}`;
				})
			)
			.selectAll("text")
			.attr("fill", "var(--muted-foreground)")
			.style("font-size", "12px");

		g.append("g")
			.call(d3.axisLeft(yScale))
			.selectAll("text")
			.attr("fill", "var(--muted-foreground)")
			.style("font-size", "12px");

		// Create line generator
		const line = d3.line<typeof parsedData[0]>()
			.x(d => xScale(d.date))
			.y(d => yScale(d.value))
			.curve(d3.curveMonotoneX);

		// Add line
		g.append("path")
			.datum(parsedData)
			.attr("fill", "none")
			.attr("stroke", color)
			.attr("stroke-width", 3)
			.attr("d", line);

		// Add dots
		g.selectAll(".dot")
			.data(parsedData)
			.enter().append("circle")
			.attr("class", "dot")
			.attr("cx", d => xScale(d.date))
			.attr("cy", d => yScale(d.value))
			.attr("r", 4)
			.attr("fill", color)
			.attr("stroke", "var(--background)")
			.attr("stroke-width", 2)
			.style("cursor", "pointer")
			.on("mouseenter", function(event: any, d: any) {
				d3.select(this)
					.transition()
					.duration(200)
					.attr("r", 6);
				
				const rect = svgRef.current!.getBoundingClientRect();
				setTooltip({
					visible: true,
					x: event.clientX - rect.left,
					y: event.clientY - rect.top,
					date: d.date.toLocaleDateString(),
					value: d.value
				});
			})
			.on("mouseleave", function() {
				d3.select(this)
					.transition()
					.duration(200)
					.attr("r", 4);
				
				setTooltip(prev => ({ ...prev, visible: false }));
			});

	}, [data, color]);

	if (data.length === 0) {
		return (
			<Card>
				<h3 className="text-lg font-semibold mb-4">{title}</h3>
				<div className="h-64 flex items-center justify-center text-muted-foreground">
					No data available
				</div>
			</Card>
		);
	}

	// Calculate recent trend
	const recent = data.slice(-7);
	const trend =
		recent.length > 1 ? recent[recent.length - 1].value - recent[0].value : 0;

	return (
		<Card className="relative">
			<div className="flex justify-between items-center mb-6">
				<h3 className="text-lg font-semibold">{title}</h3>
				<div className="flex items-center space-x-4 text-sm">
					<div className="text-muted-foreground">
						7-day trend:
						<span
							className={`ml-1 font-medium ${
								trend > 0
									? "text-success"
									: trend < 0
										? "text-destructive"
										: "text-muted-foreground"
							}`}
						>
							{trend > 0 ? "+" : ""}
							{trend.toFixed(1)}
						</span>
					</div>
				</div>
			</div>

			<div className="mb-6 relative">
				<svg
					ref={svgRef}
					width="100%"
					height="300"
					style={{ overflow: "visible" }}
				/>
				
				{tooltip.visible && (
					<div
						className="absolute z-10 bg-card border border-border rounded-lg p-2 shadow-lg pointer-events-none"
						style={{
							left: tooltip.x + 10,
							top: tooltip.y - 10,
						}}
					>
						<p className="text-sm font-medium">Date: {tooltip.date}</p>
						<p className="text-sm" style={{ color }}>
							{metricName}: {tooltip.value}
						</p>
					</div>
				)}
			</div>

			<div className="grid grid-cols-3 gap-4 text-center">
				<div>
					<div className="text-2xl font-bold text-foreground">
						{data[data.length - 1]?.value || 0}
					</div>
					<div className="text-sm text-muted-foreground">Latest</div>
				</div>
				<div>
					<div className="text-2xl font-bold text-foreground">
						{(data.reduce((sum, d) => sum + d.value, 0) / data.length).toFixed(
							1,
						)}
					</div>
					<div className="text-sm text-muted-foreground">Average</div>
				</div>
				<div>
					<div className="text-2xl font-bold text-foreground">
						{Math.max(...data.map((d) => d.value))}
					</div>
					<div className="text-sm text-muted-foreground">Peak</div>
				</div>
			</div>
		</Card>
	);
};
