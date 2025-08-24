import { MetricConfiguration } from "#/components/settings/MetricConfiguration";
import { About } from "#/components/settings/About";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import SyncProgress from "#/components/settings/syncProgress";

export const SettingsPage = () => {
	const [showSyncProgress, setShowSyncProgress] = useState(false);

	useEffect(() => {
		let unlistenPromise = listen("sync-start", () => {
			setShowSyncProgress(true);
		});

		let stopUnlistenPromise = listen("sync-complete", () => {
			setShowSyncProgress(false);
		});

		return () => {
			void unlistenPromise.then((unlisten) => unlisten());
			void stopUnlistenPromise.then((unlisten) => unlisten());
		};
	}, []);

	return (
		<div className="space-y-8 ">
			{/* Header */}
			<div>
				<h1 className="text-2xl font-bold text-foreground">Settings</h1>
				<p className="text-muted-foreground">
					Configure your habit tracking preferences and data
				</p>
			</div>

			{/* Settings Sections */}
			<div className="space-y-8">
				<MetricConfiguration />
				<About />
			</div>
			<SyncProgress
				open={showSyncProgress}
				onOpenChange={setShowSyncProgress}
			/>
		</div>
	);
};
