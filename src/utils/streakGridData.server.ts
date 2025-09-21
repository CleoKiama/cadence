import { invoke } from "@tauri-apps/api/core";
import { ChartDataSchema } from "./activityDataSchema.server";

export default async function getStreakData() {
	const date = new Date();
	const year = date.getFullYear();
	const month = date.getMonth();
	const result = await invoke<Array<unknown>>("get_current_streak_data", {
		year,
		month,
	});
	console.log("result", result);
	return result.map((item) => ChartDataSchema.parse(item));
}
