import { invoke } from "@tauri-apps/api/core";
import z from "zod";
import { ChartDataSchema } from "./activityDataSchema.server";

// get activity data for the past n days
export async function fetchHabitTrends(
	days: number,
): Promise<z.infer<typeof ChartDataSchema>[]> {
	const allData = await invoke<Array<unknown>>("get_all_analytics_data", {
		days,
	});
	const validatedData = allData.map((data) => ChartDataSchema.parse(data));
	return validatedData;
}
