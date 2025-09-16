import { useEffect, useRef, useState } from "react";
import * as d3 from "d3";
import z from "zod";
import { ChartDataSchema } from "#/utils/analytics_data";

type MetricData = z.infer<typeof ChartDataSchema>;

// Base dimensions for viewBox
const baseWidth = 500;
const baseHeight = 500;

export default function StreakGrid({ data, habitName }: MetricData) {
	const svgRef = useRef<SVGSVGElement | null>(null);
	const containerRef = useRef<HTMLDivElement | null>(null);
	const [dimensions, setDimensions] = useState({
		width: baseWidth,
		height: baseHeight,
	});

	useEffect(() => {
		const updateDimensions = () => {
			if (containerRef.current) {
				const containerWidth = containerRef.current.clientWidth;

				// Maintain aspect ratio but scale to container
				const aspectRatio = baseWidth / baseHeight;
				let newWidth = containerWidth;
				let newHeight = containerWidth / aspectRatio;

				// Ensure minimum size for readability
				const minSize = 200;
				if (newWidth < minSize) {
					newWidth = minSize;
					newHeight = minSize / aspectRatio;
				}

				setDimensions({ width: newWidth, height: newHeight });
			}
		};

		updateDimensions();

		const resizeObserver = new ResizeObserver(updateDimensions);
		if (containerRef.current) {
			resizeObserver.observe(containerRef.current);
		}

		return () => resizeObserver.disconnect();
	}, []);

	useEffect(() => {
		if (!svgRef.current || !data.length) return;

		const { width, height } = dimensions;
		const daysInWeek = 7;

		// Scale factors based on container size
		const scaleFactor = Math.min(width, height) / baseWidth;
		const dotRadius = Math.max(8, 16 * scaleFactor); // Min 8px, scales up
		const rowPadding = 80 * scaleFactor;

		// Responsive positioning
		const monthAndMetricY = height * 0.08; // 8% from top
		const labelsY = height * 0.15; // 15% from top
		const gridTop = height * 0.25; // 25% from top, where your yScale starts

		const svg = d3
			.select(svgRef.current)
			.attr("viewBox", `0 0 ${baseWidth} ${baseHeight}`)
			.attr("width", "100%")
			.attr("height", "100%")
			.style("max-width", `${width}px`)
			.style("max-height", `${height}px`);

		const xScale = d3
			.scaleLinear()
			.domain([0, daysInWeek])
			.range([30, baseWidth - 30]); //INFO: used for cx

		const yScale = d3
			.scaleLinear()
			.domain([0, Math.ceil(data.length / daysInWeek)])
			.range([gridTop, baseHeight - rowPadding]);

		const grid = data.map((d, i) => {
			const col = i % daysInWeek;
			const row = Math.floor(i / daysInWeek);
			const day = new Date(d.date).getDate();

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

		// Responsive font sizes
		const titleFontSize = Math.max(12, 20 * scaleFactor);
		const monthFontSize = Math.max(10, 16 * scaleFactor);
		const dayFontSize = Math.max(8, 14 * scaleFactor);
		const valueFontSize = Math.max(6, 12 * scaleFactor);

		// Metric label
		svg
			.append("text")
			.text(habitName)
			.attr("x", baseWidth / 2)
			.attr("y", monthAndMetricY)
			.attr("text-anchor", "middle")
			.style("font-size", `${titleFontSize}px`)
			.style("fill", "hsl(var(--foreground))")
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

		svg
			.append("text")
			.text(month)
			.attr("x", 0)
			.attr("y", monthAndMetricY)
			.attr("text-anchor", "middle")
			.style("font-size", `${monthFontSize}px`)
			.style("fill", "hsl(var(--foreground))")
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
			.attr("x", (_, i) => xScale(i))
			.attr("y", labelsY)
			.attr("text-anchor", "middle")
			.style("font-size", `${dayFontSize}px`)
			.style("fill", "hsl(var(--foreground))")
			.style("font-weight", "bold");

		svg
			.selectAll("dot")
			.data(grid)
			.enter()
			.append("circle")
			.attr("cx", (d) => xScale(d.col))
			.attr("cy", (d) => yScale(d.row))
			.attr("r", dotRadius)
			.attr("fill", "#90EE90");

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
			.style("font-size", `${valueFontSize}px`)
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

				//x2 will be half the normal distance between two dots from the start of t where the next dot would be
				if (d.col === daysInWeek - 1) return end - Math.floor(dotSpacing / 2);
				return end - dotRadius; // remove the dotradius to avoid the line going into the next circle
			}) //start next dot
			.attr("y2", (d) => yScale(d.row))
			.attr("stroke", "#90EE90")
			.attr("stroke-width", Math.max(1, 3 * scaleFactor));

		svg
			.selectAll("dot-streak-line-start")
			.data(grid.filter((d) => d.value > 0 && d.col === 0))
			.enter()
			.append("line")
			.attr("x1", (d) => {
				const dotStart = xScale(d.col);
				const lineLength = 30 * scaleFactor;
				return dotStart - lineLength;
			})
			.attr("y1", (d) => yScale(d.row))
			.attr("x2", (d) => {
				const dotStart = xScale(d.col); //start of next dot
				return dotStart - dotRadius;
			})
			.attr("y2", (d) => yScale(d.row))
			.attr("stroke", "#90EE90")
			.attr("stroke-width", Math.max(1, 3 * scaleFactor));
	}, [data, habitName, dimensions]);

	return (
		<div ref={containerRef} className="w-full h-auto min-h-[200px]">
			<svg ref={svgRef} className="w-full h-auto"></svg>
		</div>
	);
}
