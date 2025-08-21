import { Card } from "#/components/ui/card";
import { Badge } from "#/components/ui/badge";
import z from "zod";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ChooseJournalDirectory from "./selectJournalPath";
import {
	TrackedMetricsEditor,
	type MetricFormData,
} from "./TrackedMetricsEditor";

const TrackedMetricSchema = z.object({
	name: z.string(),
	active: z.boolean(),
	lastUpdated: z.string(),
	entries: z.number(),
});

const settingsSchema = z.object({
	journalFilesPath: z.string().nullable(),
	trackedMetrics: z.array(TrackedMetricSchema).optional(),
});

type Settings = z.infer<typeof settingsSchema>;

export const MetricConfiguration = () => {
	const [data, setData] = useState<Settings>();
	const [error, setError] = useState("");
	const [loading, setLoading] = useState(true);
	const [isEditing, setIsEditing] = useState(false);
	const [operationLoading, setOperationLoading] = useState(false);

	const fetchSettings = async () => {
		try {
			setLoading(true);
			const response = await invoke("get_settings");
			console.log("response", response);
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

	const handlePathChange = () => {
		// Refresh settings after path change to get updated metrics
		fetchSettings();
	};

	const handleAddMetric = async (metricData: MetricFormData) => {
		setOperationLoading(true);
		try {
			// TODO: Replace with actual backend call when ready
			await new Promise((resolve) => setTimeout(resolve, 1000)); // Simulate API call
			console.log("Adding metric:", metricData.name);

			// For now, just refresh the data
			await fetchSettings();
		} catch (error) {
			console.error("Failed to add metric:", error);
			throw error;
		} finally {
			setOperationLoading(false);
		}
	};

	const handleDeleteMetric = async (metricName: string) => {
		setOperationLoading(true);
		try {
			// TODO: Replace with actual backend call when ready
			await new Promise((resolve) => setTimeout(resolve, 500)); // Simulate API call
			console.log("Deleting metric:", metricName);

			// For now, just refresh the data
			await fetchSettings();
		} catch (error) {
			console.error("Failed to delete metric:", error);
			throw error;
		} finally {
			setOperationLoading(false);
		}
	};

	if (loading) {
		return <Card className="p-6">Loading settings...</Card>;
	}

	if (error) {
		return <Card className="p-6">Something went wrong: {error}</Card>;
	}

	return (
		<Card className="p-6">
			<ChooseJournalDirectory
				initialJournalPath={data?.journalFilesPath || null}
				onPathChange={handlePathChange}
			/>

			{data?.journalFilesPath && (
				<>
					<div className="mb-6 mt-6 flex items-center justify-between">
						<div>
							<h3 className="text-lg font-semibold">Tracked Metrics</h3>
							<p className="text-sm text-muted-foreground">
								These metrics are automatically extracted from your daily
								journal files
							</p>
						</div>
						{!isEditing && (
							<button
								onClick={() => setIsEditing(true)}
								className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 text-sm font-medium"
							>
								Edit Metrics
							</button>
						)}
					</div>

					{isEditing ? (
						<TrackedMetricsEditor
							metrics={data?.trackedMetrics || []}
							onAddMetric={handleAddMetric}
							onDeleteMetric={handleDeleteMetric}
							onCancel={() => setIsEditing(false)}
							loading={operationLoading}
						/>
					) : (
						<div className="space-y-4">
							{data?.trackedMetrics && data.trackedMetrics.length > 0 ? (
								data.trackedMetrics.map((metric, i) => (
									<div
										key={i}
										className="flex items-center justify-between p-4 border border-border rounded-lg"
									>
										<div className="flex items-center space-x-4">
											<div className="p-2 rounded-lg bg-primary text-primary-foreground">
												💻
											</div>
											<div>
												<h4 className="font-medium">{metric.name}</h4>
												<p className="text-sm text-muted-foreground">
													Field:{" "}
													<code className="px-1 py-0.5 bg-muted rounded text-xs">
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
												<div className="text-xs text-muted-foreground">
													Last:{" "}
													{new Date(metric.lastUpdated).toLocaleDateString()}
												</div>
											</div>
											<Badge variant={metric.active ? "secondary" : "default"}>
												{metric.active ? "Active" : "Inactive"}
											</Badge>
										</div>
									</div>
								))
							) : (
								<div className="p-4 text-center text-muted-foreground">
									No metrics found yet. Add some frontmatter to your journal
									files to get started.
								</div>
							)}
						</div>
					)}

					<div className="mt-6 p-4 bg-muted border border-border rounded-lg">
						<h4 className="font-medium text-foreground mb-2">How it works</h4>
						<ul className="text-sm text-muted-foreground space-y-1">
							<li>
								• Metrics are extracted from YAML frontmatter in your daily
								journal files
							</li>
							<li>• File names should follow the format: YYYY-MM-DD.md</li>
							<li>
								• Add metrics in your frontmatter like:{" "}
								<code>did_journal: 1</code>
							</li>
							<li>
								• The app automatically watches for file changes and updates
								data
							</li>
						</ul>
					</div>
				</>
			)}
		</Card>
	);
};
