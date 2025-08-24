import {
	AlertDialog,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogHeader,
	AlertDialogTitle,
} from "#/components/ui/alert-dialog";
import { useEffect, useState } from "react";
import { Loader2 } from "lucide-react";
import { listen } from "@tauri-apps/api/event";

type SyncProgress = {
	syncProgress: number;
};

export default function SyncProgress({
	open,
	onOpenChange,
}: {
	open: boolean;
	onOpenChange: (open: boolean) => void;
}) {
	const [progress, setProgress] = useState(0);

	useEffect(() => {
		if (!open) return;

		let unlistenPromise = listen<SyncProgress>("sync-progress", (event) => {
			const newProgress = event.payload.syncProgress;
			console.log("newProgress", newProgress);
			setProgress(newProgress);
		});

		return () => {
			unlistenPromise.then((unlisten) => unlisten());
		};
	}, [open]);

	useEffect(() => {
		if (!open) {
			setProgress(0);
		}
	}, [open]);

	if (!open) return null;

	return (
		<AlertDialog open={open} onOpenChange={onOpenChange}>
			<AlertDialogContent className="sm:max-w-md">
				<AlertDialogHeader>
					<AlertDialogTitle>Syncing Data</AlertDialogTitle>
					<AlertDialogDescription asChild>
						<span className="flex items-center mt-2">
							<Loader2 className="mr-2 h-4 w-4 animate-spin" />
							<span>{progress}% Complete</span>
						</span>
					</AlertDialogDescription>
				</AlertDialogHeader>
			</AlertDialogContent>
		</AlertDialog>
	);
}
