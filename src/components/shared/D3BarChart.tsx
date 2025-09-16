import { useEffect, useRef, useState } from "react";
import * as d3 from "d3";

interface DataPoint {
	date: string;
	value: number;
}

interface D3BarChartProps {
	habitName: string;
	data: DataPoint[];
	height?: number;
}

export default function D3BarChart({ 
	habitName,
	data, 
	height = 300 
}: D3BarChartProps) {
	const containerRef = useRef<HTMLDivElement>(null);
	const svgRef = useRef<SVGSVGElement>(null);
	const [dimensions, setDimensions] = useState({ width: 600, height });

	useEffect(() => {
		const updateDimensions = () => {
			if (containerRef.current) {
				const width = containerRef.current.offsetWidth;
				setDimensions({ width, height });
			}
		};

		updateDimensions();
		window.addEventListener('resize', updateDimensions);
		
		return () => window.removeEventListener('resize', updateDimensions);
	}, [height]);

	useEffect(() => {
		if (!data || data.length === 0 || !svgRef.current) return;

		const svg = d3.select(svgRef.current);
		svg.selectAll("*").remove();

		const margin = { top: 20, right: 30, bottom: 40, left: 40 };
		const innerWidth = dimensions.width - margin.left - margin.right;
		const innerHeight = dimensions.height - margin.top - margin.bottom;

		const container = svg
			.attr("width", dimensions.width)
			.attr("height", dimensions.height);

		const g = container
			.append("g")
			.attr("transform", `translate(${margin.left},${margin.top})`);

		const xScale = d3
			.scaleBand()
			.domain(data.map(d => d.date))
			.range([0, innerWidth])
			.padding(0.1);

		const yScale = d3
			.scaleLinear()
			.domain([0, d3.max(data, d => d.value) || 0])
			.nice()
			.range([innerHeight, 0]);

		g.append("g")
			.attr("class", "x-axis")
			.attr("transform", `translate(0,${innerHeight})`)
			.call(d3.axisBottom(xScale))
			.selectAll("text")
			.style("fill", "var(--primary)")
			.style("font-size", "12px");

		g.append("g")
			.attr("class", "y-axis")
			.call(d3.axisLeft(yScale))
			.selectAll("text")
			.style("fill", "var(--foreground)")
			.style("font-size", "12px");

		g.selectAll(".domain")
			.style("stroke", "var(--border)");

		g.selectAll(".tick line")
			.style("stroke", "var(--border)");

		const tooltip = d3
			.select("body")
			.append("div")
			.attr("class", "d3-tooltip")
			.style("position", "absolute")
			.style("visibility", "hidden")
			.style("background", "var(--card)")
			.style("border", "1px solid var(--border)")
			.style("border-radius", "4px")
			.style("padding", "8px")
			.style("font-size", "12px")
			.style("color", "var(--foreground)")
			.style("box-shadow", "0 2px 4px rgba(0,0,0,0.1)")
			.style("z-index", "1000");

		g.selectAll(".bar")
			.data(data)
			.enter()
			.append("rect")
			.attr("class", "bar")
			.attr("x", d => xScale(d.date) || 0)
			.attr("y", d => yScale(d.value))
			.attr("width", Math.min(30, xScale.bandwidth()))
			.attr("height", d => innerHeight - yScale(d.value))
			.style("fill", "var(--primary)")
			.style("cursor", "pointer")
			.on("mouseover", (event, d) => {
				tooltip
					.style("visibility", "visible")
					.html(`Date: ${d.date}<br/>Value: ${d.value}`);
				
				d3.select(event.currentTarget)
					.style("fill", "var(--primary)")
					.style("opacity", "0.8");
			})
			.on("mousemove", (event) => {
				tooltip
					.style("top", (event.pageY - 10) + "px")
					.style("left", (event.pageX + 10) + "px");
			})
			.on("mouseout", (event) => {
				tooltip.style("visibility", "hidden");
				d3.select(event.currentTarget)
					.style("opacity", "1");
			});

		return () => {
			d3.selectAll(".d3-tooltip").remove();
		};
	}, [data, dimensions]);

	return (
		<div>
			<h2>{habitName}</h2>
			<div ref={containerRef} className="w-full">
				<svg ref={svgRef} className="block" />
			</div>
		</div>
	);
}