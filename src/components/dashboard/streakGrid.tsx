import { useEffect, useRef, useState } from "react";
import * as d3 from "d3";
import z from "zod";
import { ChartDataSchema } from "#/utils/activityDataSchema.server";
import { drawSvg } from "./utils/drawSvg";

type MetricData = z.infer<typeof ChartDataSchema>;

// Base dimensions for viewBox
const baseWidth = 500;
const baseHeight = 700;

export default function StreakGrid({ data, habitName }: MetricData) {
	const svgRef = useRef<SVGSVGElement | null>(null);
	const containerRef = useRef<HTMLDivElement | null>(null);
	const [dimensions, setDimensions] = useState({
		width: baseWidth,
		height: baseHeight,
	});
	const [currentDate, setCurrentDate] = useState(new Date());

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
		const metricData = {
			habitName,
			data,
		};
		drawSvg({
			baseWidth,
			baseHeight,
			dimensions,
			currentDate,
			metricData,
			svgRef: svgRef.current,
		});
	}, [data, habitName, dimensions]);

	return (
		<div ref={containerRef} className="w-full h-auto min-h-[200px]">
			<svg ref={svgRef} className="w-full h-auto"></svg>
		</div>
	);
}
