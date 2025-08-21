import React from "react";

export type ViewMode = "dashboard" | "analytics" | "settings";

interface NavigationItem {
	id: ViewMode;
	label: string;
	icon: string;
}
// Temporary icon components
const BarChart3Icon = () => (
	<svg
		className="h-5 w-5"
		fill="none"
		stroke="currentColor"
		viewBox="0 0 24 24"
	>
		<path d="M3 3v18h18" />
		<path d="M18 17V9" />
		<path d="M13 17V5" />
		<path d="M8 17v-3" />
	</svg>
);

const TrendingUpIcon = () => (
	<svg
		className="h-5 w-5"
		fill="none"
		stroke="currentColor"
		viewBox="0 0 24 24"
	>
		<polyline points="23,6 13.5,15.5 8.5,10.5 1,18" />
		<polyline points="17,6 23,6 23,12" />
	</svg>
);

const SettingsIcon = () => (
	<svg
		className="h-5 w-5"
		fill="none"
		stroke="currentColor"
		viewBox="0 0 24 24"
	>
		<circle cx="12" cy="12" r="3" />
		<path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1 1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
	</svg>
);

interface NavigationProps {
	activeView: ViewMode;
	onViewChange: (view: ViewMode) => void;
}

const navigationItems: NavigationItem[] = [
	{ id: "dashboard", label: "Dashboard", icon: "bar-chart-3" },
	{ id: "analytics", label: "Analytics", icon: "trending-up" },
	{ id: "settings", label: "Settings", icon: "settings" },
];

export const Navigation: React.FC<NavigationProps> = ({
	activeView,
	onViewChange,
}) => {
	const getIcon = (iconName: string) => {
		switch (iconName) {
			case "bar-chart-3":
				return <BarChart3Icon />;
			case "trending-up":
				return <TrendingUpIcon />;
			case "settings":
				return <SettingsIcon />;
			default:
				return <BarChart3Icon />;
		}
	};

	return (
		<nav className="bg-card border-r border-border h-full min-h-screen">
			<div className="p-4">
				<ul className="space-y-2">
					{navigationItems.map((item) => (
						<li key={item.id}>
							<button
								onClick={() => onViewChange(item.id)}
								className={`w-full flex items-center space-x-3 px-4 py-3 rounded-lg transition-all duration-200 ${
									activeView === item.id
										? "bg-primary text-primary-foreground shadow-md"
										: "text-foreground hover:bg-muted hover:scale-105"
								}`}
							>
								{getIcon(item.icon)}
								<span className="font-medium">{item.label}</span>
							</button>
						</li>
					))}
				</ul>
			</div>
		</nav>
	);
};
