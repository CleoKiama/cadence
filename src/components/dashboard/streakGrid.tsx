import { useEffect, useRef } from "react";
import * as d3 from "d3";
import z from "zod";
import { ChartDataSchema } from "#/utils/analytics_data";

type MetricData = z.infer<typeof ChartDataSchema>;

const width = 500;
const height = 500;
const dotRadius = 16;
const rowPadding = 80;

export default function StreakGrid({ data, habitName }: MetricData) {
	const svgRef = useRef<SVGSVGElement | null>(null);

	useEffect(() => {
		const daysInWeek = 7;
		const monthAndMetricY = height * 0.08; // 8% from top
		const labelsY = height * 0.15; // 15% from top
		const gridTop = height * 0.25; // 25% from top, where your yScale starts

		const svg = d3
			.select(svgRef.current)
			.attr("width", width)
			.attr("height", height);

		const xScale = d3
			.scaleLinear()
			.domain([0, daysInWeek])
			.range([30, width - 30]); //INFO: used for cx

		const yScale = d3
			.scaleLinear()
			.domain([0, Math.ceil(data.length / daysInWeek)])
			.range([gridTop, height - rowPadding]);

		const grid = data.map((d, i) => {
			const col = i % daysInWeek;
			const row = Math.floor(i / daysInWeek);
			const day = new Date(d.date).getDay();
			return {
				...d,
				row,
				col,
				i,
				day,
			};
		});

		//INFO:: remove everything to avoid duplicates on rerenders
		svg.selectAll("*").remove();
		// Metric label
		svg
			.append("text")
			.text(habitName)
			.attr("x", width / 2)
			.attr("y", monthAndMetricY)
			.attr("text-anchor", "middle")
			.style("font-size", "20px")
			.style("fill", "#333")
			.style("font-weight", "bold");

		// Month label
		let month = new Date(data[0].date)
			.toLocaleDateString("en-US", {
				month: "short",
				year: "numeric",
			})
			.toUpperCase()
			.split(" ")
			.join("\n");

		console.log("month", month);

		svg
			.append("text")
			.text(month)
			.attr("x", 0)
			.attr("y", monthAndMetricY)
			.attr("text-anchor", "middle")
			.style("font-size", "16px")
			.style("fill", "#333")
			.style("font-weight", "bold");

		// Day labels
		const days: string[] = [];
		for (let i = 0; i < 7; i++) {
			const day = new Date(grid[i].date).toLocaleDateString("en-US", {
				weekday: "short",
			});
			days.push(day);
		}

		svg
			.selectAll("day-label")
			.data(days)
			.enter()
			.append("text")
			.text((d) => d)
			.attr("x", (d, i) => xScale(i))
			.attr("y", labelsY)
			.attr("text-anchor", "middle")
			.style("font-size", "14px")
			.style("fill", "#333")
			.style("font-weight", "bold");

		svg
			.selectAll("dot")
			.data(grid)
			.enter()
			.append("circle")
			.attr("cx", (d) => xScale(d.col))
			.attr("cy", (d) => yScale(d.row))
			.attr("r", dotRadius)
			.attr("fill", "#69b3a2");

		svg
			.selectAll("dot-value")
			.data(grid)
			.enter()
			.append("text")
			.text((d) => d.day)
			.attr("x", (d) => xScale(d.col))
			.attr("y", (d) => yScale(d.row))
			.attr("text-anchor", "middle")
			.attr("dy", ".35em")
			.style("font-size", "12px")
			.style("fill", "white");

		svg
			.selectAll("dot-streak-line")
			.data(grid.filter((d) => d.value > 0 && d.i < grid.length - 1)) //remove values with 0 and last value of the month
			.enter()
			.append("line")
			.attr("x1", (d) => {
				const start = xScale(d.col);
				return start + dotRadius;
			})
			.attr("y1", (d) => yScale(d.row))
			.attr("x2", (d) => {
				const end = xScale(d.col + 1); //start of next dot
				const dotSpacing = xScale(1) - xScale(0); //Distance between centers

				if (d.col === daysInWeek - 1) return end - Math.floor(dotSpacing / 2);
				return end;
			}) //start next dot
			.attr("y2", (d) => yScale(d.row))
			.attr("stroke", "#69b3a2")
			.attr("stroke-width", 3);

		svg
			.selectAll("dot-streak-line-start")
			.data(grid.filter((d) => d.value > 0 && d.col === 0))
			.enter()
			.append("line")
			.attr("x1", (d) => {
				const dotStart = xScale(d.col);
				const lineLength = 30;
				return dotStart - lineLength;
			})
			.attr("y1", (d) => yScale(d.row))
			.attr("x2", (d) => {
				const dotStart = xScale(d.col); //start of next dot
				return dotStart - dotRadius;
			})
			.attr("y2", (d) => yScale(d.row))
			.attr("stroke", "#69b3a2")
			.attr("stroke-width", 3);
	}, []);

	return <svg ref={svgRef}></svg>;
}
