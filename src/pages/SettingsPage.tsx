import { MetricConfiguration } from "../components/settings/MetricConfiguration";
import { About } from "../components/settings/About";

export const SettingsPage = () => {
	return (
		<div className="space-y-8">
			{/* Header */}
			<div>
				<h1 className="text-2xl font-bold text-[var(--color-foreground)]">
					Settings
				</h1>
				<p className="text-[var(--color-muted-foreground)]">
					Configure your habit tracking preferences and data
				</p>
			</div>

			{/* Settings Sections */}
			<div className="space-y-8">
				<MetricConfiguration />
				<About />
			</div>
		</div>
	);
};

