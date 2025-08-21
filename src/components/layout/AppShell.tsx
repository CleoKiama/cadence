import React from "react";
import { Header } from "./Header";
import { Navigation } from "./Navigation";

interface AppShellProps {
	children: React.ReactNode;
	activeView: ViewMode;
	onViewChange: (view: ViewMode) => void;
}

type ViewMode = "dashboard" | "analytics" | "settings";

export const AppShell: React.FC<AppShellProps> = ({
	children,
	activeView,
	onViewChange,
}) => {
	return (
		<div className="min-h-screen bg-background text-foreground">
			<Header />
			<div className="flex h-[calc(100vh-73px)]">
				{/* Sidebar Navigation */}
				<div className="w-64 hidden md:block">
					<Navigation activeView={activeView} onViewChange={onViewChange} />
				</div>

				{/* Main Content */}
				<main className="flex-1 overflow-auto pb-20 md:pb-0">
					<div className="p-6">{children}</div>
				</main>
			</div>

			{/* Mobile Navigation */}
			<div className="md:hidden fixed bottom-0 left-0 right-0 bg-card border-t border-border z-50">
				<div className="grid grid-cols-3 py-2">
					{[
						{ id: "dashboard" as ViewMode, label: "Dashboard", icon: "📊" },
						{ id: "analytics" as ViewMode, label: "Analytics", icon: "📈" },
						{ id: "settings" as ViewMode, label: "Settings", icon: "⚙" },
					].map((item) => (
						<button
							key={item.id}
							onClick={() => onViewChange(item.id)}
							className={`flex flex-col items-center py-2 px-4 transition-colors ${
								activeView === item.id
									? "text-primary"
									: "text-muted-foreground"
							}`}
						>
							<span className="text-lg mb-1">{item.icon}</span>
							<span className="text-xs">{item.label}</span>
						</button>
					))}
				</div>
			</div>
		</div>
	);
};

