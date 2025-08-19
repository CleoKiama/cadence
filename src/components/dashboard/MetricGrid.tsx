import { useState, useEffect } from "react";
import { MetricCard } from "./MetricCard";
import { z } from "zod";
import { invoke } from "@tauri-apps/api/core";

const MetricSummarySchema = z.object({
	name: z.string(),
	displayName: z.string(),
	currentStreak: z.number(),
	longestStreak: z.number(),
	weeklyAverage: z.number(),
	monthlyTotal: z.number(),
	lastUpdated: z.string(),
	trend: z.enum(["up", "down", "stable"]),
});

const MetricGridSchema = z.array(MetricSummarySchema);

export type MetricSummary = z.infer<typeof MetricSummarySchema>;

export const MetricGrid = () => {
	const [metrics, setMetrics] = useState<MetricSummary[]>([]);

	useEffect(() => {
		const result = invoke("get_dashboard_metrics");
		result
			.then((res) => {
				const parseResult = MetricGridSchema.safeParse(res);
				if (!parseResult.success) {
					console.error("Failed to parse metrics:", parseResult.error);
					console.log("Response data:", res);
					return;
				}
				console.log("parseResult", parseResult);
				setMetrics(parseResult.data);
			})
			.catch((error) => {
				console.error("Error fetching metrics:", error);
			});
	}, []);

	return (
		<div>
			<h2 className="text-xl font-semibold mb-6 text-[var(--color-foreground)]">
				Your Habits
			</h2>
			<div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
				{metrics.map((metric, i) => (
					<MetricCard key={i} {...metric} />
				))}
			</div>
		</div>
	);
};
