import { Card } from "#/components/ui/card";
import { Badge } from "#/components/ui/badge";
import z from "zod";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ChooseJournalDirectory from "./selectJournalPath";
import { TrackedMetricsEditor } from "./TrackedMetricsEditor";
import { Button } from "../ui/button";
import { AddMetricForm } from "./AddMetric";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
	DialogClose,
} from "#/components/ui/dialog";

const TrackedMetricSchema = z.object({
	name: z.string(),
	active: z.boolean(),
	lastUpdated: z.string(),
	entries: z.number(),
});

export const trackedMetricsShema = z.array(TrackedMetricSchema);

const settingsSchema = z.object({
	journalFilesPath: z.string().nullable(),
	trackedMetrics: trackedMetricsShema.nullable(),
});

type Settings = z.infer<typeof settingsSchema>;

export const metricFormSchema = z.object({
	name: z
		.string()
		.min(1, "Metric name is required")
		.max(50, "Metric name must be 50 characters or less")
		.regex(
			/^[a-zA-Z0-9_]+$/,
			"Metric name can only contain letters, numbers, and underscores",
		)
		.refine(
			(name) => !name.startsWith("_") && !name.endsWith("_"),
			"Metric name cannot start or end with underscores",
		),
});

export const MetricConfiguration = () => {
	const [data, setData] = useState<Settings>();
	const [error, setError] = useState("");
	const [loading, setLoading] = useState(true);

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
		fetchSettings();
	};

	const handleAddMetric = async (newMetric: string) => {
		invoke("add_metric", { metricName: newMetric })
			.then(() => {
				console.log("Metric added successfully");
				void fetchSettings();
			})
			.catch((err) => console.error("Error adding metric:", err));
	};

	const handleDeleteMetric = (metricName: string) => {
		invoke("delete_metric", { metricName })
			.then(() => {
				void fetchSettings();
			})
			.catch((err) => console.error("Error Deleting Metric:", err));
	};
	const handleMetricUpdate = ({
		newName,
		prevName,
	}: {
		newName: string;
		prevName: string;
	}) => {
		invoke("udpate_metric", {
			prevName,
			newName,
		})
			.then(() => {
				void fetchSettings();
			})
			.catch((e) => console.error("Error updating metric:", e));
	};

	if (loading) {
		//TODO: replace with like backed backed loading thing
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
					</div>
					<AddMetricForm
						metrics={data.trackedMetrics || []}
						onMetricUpdate={handleAddMetric}
					/>

					{
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
											<TrackedMetricsEditor
												onMetricUpdate={handleMetricUpdate}
												name={metric.name}
											/>
											<DeleteMetric
												metricName={metric.name}
												onDelete={handleDeleteMetric}
											/>
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
					}

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

const DeleteMetric = ({
	metricName,
	onDelete,
}: {
	onDelete: (metricName: string) => void;
	metricName: string;
}) => {
	return (
		<Dialog>
			<DialogTrigger>
				<Button variant="destructive" className="w-full cursor-pointer">
					Delete
				</Button>
			</DialogTrigger>
			<DialogContent>
				<DialogHeader>
					<DialogTitle>Delete Metric</DialogTitle>
					<DialogDescription className="text-muted-foreground">
						Are you sure you want to delete the metric: {metricName}
					</DialogDescription>
				</DialogHeader>
				<DialogFooter>
					<DialogClose asChild>
						<div className="flex items-center gap-3">
							<Button onClick={() => onDelete(metricName)}>Confirm</Button>
							<Button variant="secondary">Cancel</Button>
						</div>
					</DialogClose>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
};
