import { useEffect, useState } from "react";
import {
	BarChart,
	Bar,
	XAxis,
	YAxis,
	Tooltip,
	Legend,
	ResponsiveContainer,
} from "recharts";
import { Skeleton } from "#/components/ui/skeleton";
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
				<RenderBarChart key={i} habitName={item.habitName} data={item.data} />
			))}
		</div>
	);
}

const RenderBarChart = ({
	habitName,
	data,
}: {
	habitName: string;
	data: Array<z.infer<typeof MetricSchema>>;
}) => {
	return (
		<div>
			<h2>{habitName}</h2>
			<ResponsiveContainer width="100%" height={300}>
				<BarChart width={600} height={300} data={data}>
					<XAxis dataKey="date" stroke="var(--primary)" />
					<YAxis />
					<Tooltip
						wrapperStyle={{ width: 100, backgroundColor: "var(--card)" }}
					/>
					<Legend
						width={100}
						wrapperStyle={{
							top: 40,
							right: 20,
							backgroundColor: "var(--muted)",
							border: "1px solid var(--border)",
							borderRadius: 3,
							lineHeight: "40px",
						}}
					/>
					<Bar dataKey="value" fill="var(--primary)" barSize={30} />
				</BarChart>
			</ResponsiveContainer>
		</div>
	);
};
