import { useEffect, useState } from "react";
import { Skeleton } from "#/components/ui/skeleton";
import D3BarChart from "#/components/shared/D3BarChart";
import z from "zod";
import { invoke } from "@tauri-apps/api/core";

const MetricSchema = z.object({
	value: z.number(),
	date: z.string(), // ISO date string
});

const DataSchema = z.array(
	z.object({
		habitName: z.string(),
		data: z.array(MetricSchema),
	}),
).nullable();

type Data = z.infer<typeof DataSchema>;

export default function RecentActivity() {
	const [data, setData] = useState<Data>(null);
	const [loading, setLoading] = useState(true);

	useEffect(() => {
		const fetchData = async () => {
			try {
				setLoading(true);
				const resData = await invoke<Data>("get_recent_activity");
				const result = DataSchema.safeParse(resData);
				if (!result.success) {
					console.error("Data validation failed:", result.error);
					setData(null);
					return;
				}
				setData(result.data);
			} catch (error) {
				console.error("Error fetching recent activity:", error);
				setData(null);
			} finally {
				setLoading(false);
			}
		};

		fetchData();
	}, []);

	if (loading) {
		return (
			<div>
				<Skeleton className="h-6 w-40 mb-6" />
				<div className="space-y-4">
					<Skeleton className="h-8 w-48" />
					<Skeleton className="h-64 w-full rounded-lg" />
				</div>
			</div>
		);
	}

	// Don't render if no data
	if (!data || data.length === 0) {
		return null;
	}

	return (
		<div>
			<h2 className="text-xl font-semibold mb-6 text-foreground">
				Recent Activity
			</h2>
			{data.map((item, i) => (
				<D3BarChart key={i} habitName={item.habitName} data={item.data} />
			))}
		</div>
	);
}
