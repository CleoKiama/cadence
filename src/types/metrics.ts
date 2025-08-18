export interface HeatmapDataPoint {
	date: string;
	count: number;
	level: 0 | 1 | 2 | 3 | 4; // Intensity levels for coloring
}
