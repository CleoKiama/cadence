import { Card } from "../shared/Card";
import { Badge } from "../shared/Badge";
import z from "zod";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ChooseJournalDirectory from "./selectJournalPath";

const TrackedMetric = z.object({
	name: z.string(),
	active: z.boolean(),
	lastUpdated: z.string(),
	entries: z.number(),
});

const settingsSchema = z.object({
	journalFilesPath: z.string().nullable(),
	trackedMetrics: z.array(TrackedMetric).optional(),
});

type Settings = z.infer<typeof settingsSchema>;

export const MetricConfiguration = () => {
	const [data, setData] = useState<Settings>();
	const [error, setError] = useState("");
	const [loading, setLoading] = useState(true);

	const fetchSettings = async () => {
		try {
			setLoading(true);
			const response = await invoke("get_settings");
			const result = settingsSchema.safeParse(response);
			if (!result.success) {
				console.error("error validating data", result.error);
				throw new Error(result.error.message);
			}
			setData(result.data);
			setError("");
		} catch (err) {
			console.error("err in settings", err);
			if (err instanceof Error) setError(err.message);
		} finally {
			setLoading(false);
		}
	};

	useEffect(() => {
		fetchSettings();
	}, []);

	const handlePathChange = (newPath: string | null) => {
		// Refresh settings after path change to get updated metrics
		fetchSettings();
	};

	if (loading) {
		return <Card>Loading settings...</Card>;
	}

	if (error) {
		return <Card>Something went wrong: {error}</Card>;
	}

	return (
		<Card>
			<ChooseJournalDirectory 
				initialJournalPath={data?.journalFilesPath || null} 
				onPathChange={handlePathChange}
			/>

			{data?.journalFilesPath && (
				<>
					<div className="mb-6 mt-6">
						<h3 className="text-lg font-semibold">Tracked Metrics</h3>
						<p className="text-sm text-[var(--color-muted-foreground)]">
							These metrics are automatically extracted from your daily journal
							files
						</p>
					</div>

					<div className="space-y-4">
						{data?.trackedMetrics && data.trackedMetrics.length > 0 ? (
							data.trackedMetrics.map((metric, i) => (
								<div
									key={i}
									className="flex items-center justify-between p-4 border border-[var(--color-border)] rounded-lg"
								>
									<div className="flex items-center space-x-4">
										<div className="p-2 rounded-lg bg-[var(--color-primary)] text-[var(--color-primary-foreground)]">
											💻
										</div>
										<div>
											<h4 className="font-medium">{metric.name}</h4>
											<p className="text-sm text-[var(--color-muted-foreground)]">
												Field:{" "}
												<code className="px-1 py-0.5 bg-[var(--color-muted)] rounded text-xs">
													{metric.name}
												</code>
											</p>
										</div>
									</div>

									<div className="flex items-center space-x-4">
										<div className="text-right">
											<div className="text-sm font-medium">
												{metric.entries} entries
											</div>
											<div className="text-xs text-[var(--color-muted-foreground)]">
												Last: {new Date(metric.lastUpdated).toLocaleDateString()}
											</div>
										</div>
										<Badge variant={metric.active ? "success" : "default"}>
											{metric.active ? "Active" : "Inactive"}
										</Badge>
									</div>
								</div>
							))
						) : (
							<div className="p-4 text-center text-[var(--color-muted-foreground)]">
								No metrics found yet. Add some frontmatter to your journal files to get started.
							</div>
						)}
					</div>

					<div className="mt-6 p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
						<h4 className="font-medium text-blue-900 dark:text-blue-100 mb-2">
							How it works
						</h4>
						<ul className="text-sm text-blue-800 dark:text-blue-200 space-y-1">
							<li>
								• Metrics are extracted from YAML frontmatter in your daily journal
								files
							</li>
							<li>• File names should follow the format: YYYY-MM-DD.md</li>
							<li>
								• Add metrics in your frontmatter like: <code>did_journal: 1</code>
							</li>
							<li>
								• The app automatically watches for file changes and updates data
							</li>
						</ul>
					</div>
				</>
			)}
		</Card>
	);
};
