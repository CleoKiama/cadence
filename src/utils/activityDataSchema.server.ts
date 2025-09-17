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
