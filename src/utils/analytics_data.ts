import { invoke } from "@tauri-apps/api/core";
import z from "zod";

export const ChartDataSchema = z.object({
	habitName: z.string(),
	data: z.array(
		z.object({
			date: z.string(),
			value: z.number(),
		}),
	),
});

export async function fetchHabitTrends(
	days: number,
): Promise<z.infer<typeof ChartDataSchema>[]> {
	const allData = await invoke<Array<unknown>>("get_all_analytics_data", {
		days,
	});
	const validatedData = allData.map((data) => ChartDataSchema.parse(data));
	return validatedData;
}
