import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

const analyticsSummarySchema = z.object({
  totalHabits: z.number(),
  completionRate: z.number(),
  activeDays: z.number(),
  longestStreak: z.number(),
});

const weeklyActivity = z.array(
  z.object({
    date: z.string(),
    value: z.number(),
  }),
);
export type AnalyticsSummary = z.infer<typeof analyticsSummarySchema>;
export type WeeklyActivity = z.infer<typeof weeklyActivity>;

export async function getAnalyticsSummary() {
  const result = await invoke<unknown>("get_analytics_summary");
  return analyticsSummarySchema.parse(result);
}

export async function getWeeklyAcitivity() {
  const result = await invoke<unknown>("get_weekly_activity");
  return weeklyActivity.parse(result);
}
