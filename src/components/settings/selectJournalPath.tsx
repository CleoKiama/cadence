import { Button } from "#/components/ui/button";
import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { Loader2 } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

export default function ChooseJournalDirectory({
	initialJournalPath = null,
	onPathChange,
}: {
	initialJournalPath: string | null;
	onPathChange?: (newPath: string | null) => void;
}) {
	const [journalPath, setJournalPath] = useState<string | null>(
		initialJournalPath,
	);
	const [isSelecting, setIsSelecting] = useState(false);
	const [isUpdating, setIsUpdating] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [success, setSuccess] = useState<string | null>(null);

	const handlePathChange = async () => {
		setIsSelecting(true);
		setError(null);
		setSuccess(null);

		try {
			const selectedPath = await open({
				multiple: false,
				directory: true,
			});

			if (selectedPath) {
				console.log("Selected path:", selectedPath);
				setJournalPath(selectedPath);
				
				// Update the backend with the new path
				setIsUpdating(true);
				await invoke("set_journal_files_path", { path: selectedPath });
				
				setSuccess("Journal path updated successfully! File watcher restarted.");
				onPathChange?.(selectedPath);
			}
		} catch (err) {
			console.error("Error setting journal path:", err);
			setError(err instanceof Error ? err.message : "Failed to update journal path");
		} finally {
			setIsSelecting(false);
			setIsUpdating(false);
			
			// Clear messages after 3 seconds
			setTimeout(() => {
				setError(null);
				setSuccess(null);
			}, 3000);
		}
	};

	return (
		<div className="space-y-4">
			<h3 className="text-lg font-semibold">Select Journal Path</h3>
			<p className="text-sm text-muted-foreground">
				Choose the path where your journal files are stored. This will help in
				automatically extracting metrics from your journal entries.
			</p>
			
			{journalPath && (
				<div className="p-3 bg-muted rounded-lg">
					<p className="text-sm font-medium">Current path:</p>
					<p className="text-sm text-muted-foreground break-all">{journalPath}</p>
				</div>
			)}

			{error && (
				<div className="p-3 bg-muted border border-destructive rounded-lg">
					<p className="text-sm text-destructive">{error}</p>
				</div>
			)}

			{success && (
				<div className="p-3 bg-muted border border-success rounded-lg">
					<p className="text-sm text-success">{success}</p>
				</div>
			)}

			<Button
				variant="default"
				className="cursor-pointer"
				disabled={isSelecting || isUpdating}
				onClick={handlePathChange}
			>
				<span className="flex items-center gap-2">
					{(isSelecting || isUpdating) && <Loader2 className="animate-spin" />}
					{isUpdating 
						? "Updating journal path..." 
						: isSelecting 
							? "Selecting directory..." 
							: "Select your journal Directory"
					}
				</span>
			</Button>
		</div>
	);
}
