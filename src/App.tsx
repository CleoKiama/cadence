import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ThemeProvider } from "./hooks/useTheme";
import {
	NavigationProvider,
	useNavigationContext,
} from "./contexts/NavigationContext";
import { AppShell } from "./components/layout/AppShell";
import { Dashboard } from "./pages/Dashboard";
import { Analytics } from "./pages/Analytics";
import { SettingsPage } from "./pages/SettingsPage";
import "./App.css";

function AppContent() {
	const { activeView, setActiveView } = useNavigationContext();
	const [isJournalConfigured, setIsJournalConfigured] = useState<
		boolean | null
	>(null);

	useEffect(() => {
		const checkJournalPath = async () => {
			try {
				const isConfigured = await invoke<boolean>(
					"is_journal_path_configured",
				);
				setIsJournalConfigured(isConfigured);
				if (!isConfigured) {
					setActiveView("settings");
				}
			} catch (error) {
				console.error("Failed to check journal path:", error);
				setIsJournalConfigured(false);
				setActiveView("settings");
			}
		};

		checkJournalPath();
	}, [setActiveView]);

	const renderCurrentView = () => {
		switch (activeView) {
			case "dashboard":
				return <Dashboard habitName="Example habit" />;
			case "analytics":
				return <Analytics />;
			case "settings":
				return <SettingsPage />;
			default:
				return <Dashboard habitName="dsa_problems_solved" />;
		}
	};

	if (isJournalConfigured === null) {
		return (
			<div className="flex items-center justify-center h-screen">
				<div className="text-lg">Loading...</div>
			</div>
		);
	}

	return (
		<AppShell activeView={activeView} onViewChange={setActiveView}>
			{renderCurrentView()}
		</AppShell>
	);
}

function App() {
	return (
		<ThemeProvider>
			<NavigationProvider>
				<AppContent />
			</NavigationProvider>
		</ThemeProvider>
	);
}

export default App;
