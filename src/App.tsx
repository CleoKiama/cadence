import { useState } from "react";
import { ViewMode } from "./types/navigation";
import { ThemeProvider } from "./hooks/useTheme";
import { useMetrics } from "./hooks/useMetrics";
import { AppShell } from "./components/layout/AppShell";
import { Dashboard } from "./pages/Dashboard";
import { Analytics } from "./pages/Analytics";
import { SettingsPage } from "./pages/SettingsPage";
import { LoadingSpinner } from "./components/shared/LoadingSpinner";
import "./App.css";

function App() {
	const [activeView, setActiveView] = useState<ViewMode>("dashboard");
	const { summary, chartData, heatmapData, loading } = useMetrics();

	const renderCurrentView = () => {
		if (loading) {
			return (
				<div className="flex items-center justify-center h-64">
					<LoadingSpinner size="lg" />
				</div>
			);
		}

		switch (activeView) {
			case "dashboard":
				return (
					<Dashboard
						metrics={summary}
						chartData={chartData}
						habitName="dsa_problems_solved"
					/>
				);
			case "analytics":
				return (
					<Analytics
						metrics={summary}
						chartData={chartData}
						heatmapData={heatmapData}
					/>
				);
			case "settings":
				return <SettingsPage metrics={summary} />;
			default:
				return (
					<Dashboard
						metrics={summary}
						chartData={chartData}
						habitName="dsa_problems_solved"
					/>
				);
		}
	};

	return (
		<ThemeProvider>
			<AppShell activeView={activeView} onViewChange={setActiveView}>
				{renderCurrentView()}
			</AppShell>
		</ThemeProvider>
	);
}

export default App;
