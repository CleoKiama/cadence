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
);

type Data = z.infer<typeof DataSchema>;

export default function RecentActivity() {
	const [data, setData] = useState<Data>([]);
	useEffect(() => {
		invoke("get_recent_activity").then((resData) => {
			const result = DataSchema.safeParse(resData);
			if (!result.success) {
				console.error("Data validation failed:", result.error);
				console.log(result.data);
				return;
			}
			setData(result.data);
		});
	}, []);

	return (
		<div>
			<h2 className="text-xl font-semibold mb-6 text-[var(--color-foreground)]">
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
					<XAxis dataKey="date" stroke="#8884d8" />
					<YAxis />
					<Tooltip wrapperStyle={{ width: 100, backgroundColor: "#ccc" }} />
					<Legend
						width={100}
						wrapperStyle={{
							top: 40,
							right: 20,
							backgroundColor: "#f5f5f5",
							border: "1px solid #d5d5d5",
							borderRadius: 3,
							lineHeight: "40px",
						}}
					/>
					<Bar dataKey="value" fill="#8884d8" barSize={30} />
				</BarChart>
			</ResponsiveContainer>
		</div>
	);
};
