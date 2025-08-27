import React from "react";
import { BarChart3, TrendingUp, Settings } from "lucide-react";

export type ViewMode = "dashboard" | "analytics" | "settings";

interface NavigationItem {
	id: ViewMode;
	label: string;
	icon: React.ComponentType<{ className?: string }>;
}

interface NavigationProps {
	activeView: ViewMode;
	onViewChange: (view: ViewMode) => void;
}

const navigationItems: NavigationItem[] = [
	{ id: "dashboard", label: "Dashboard", icon: BarChart3 },
	{ id: "analytics", label: "Analytics", icon: TrendingUp },
	{ id: "settings", label: "Settings", icon: Settings },
];

export const Navigation: React.FC<NavigationProps> = ({
	activeView,
	onViewChange,
}) => {
	return (
		<nav className="bg-card border-r border-border h-full min-h-screen">
			<div className="p-4">
				<ul className="space-y-2">
					{navigationItems.map((item) => {
						const IconComponent = item.icon;
						return (
							<li key={item.id}>
								<button
									onClick={() => onViewChange(item.id)}
									className={`w-full flex items-center space-x-3 px-4 py-3 rounded-lg transition-all duration-200 ${
										activeView === item.id
											? "bg-primary text-primary-foreground shadow-md"
											: "text-foreground hover:bg-muted hover:scale-105"
									}`}
								>
									<IconComponent className="h-5 w-5" />
									<span className="font-medium">{item.label}</span>
								</button>
							</li>
						);
					})}
				</ul>
			</div>
		</nav>
	);
};
