import { useState } from "react";
import { ViewMode } from "./types/navigation";
import { ThemeProvider } from "./hooks/useTheme";
import { AppShell } from "./components/layout/AppShell";
import { Dashboard } from "./pages/Dashboard";
import { Analytics } from "./pages/Analytics";
import { SettingsPage } from "./pages/SettingsPage";
import "./App.css";

function App() {
	const [activeView, setActiveView] = useState<ViewMode>("dashboard");

	const renderCurrentView = () => {
		switch (activeView) {
			case "dashboard":
				return <Dashboard habitName="dsa_problems_solved" />;
			case "analytics":
				return <Analytics />;
			case "settings":
				return <SettingsPage />;
			default:
				return <Dashboard habitName="dsa_problems_solved" />;
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
