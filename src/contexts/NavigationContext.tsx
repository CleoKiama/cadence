import React, { createContext, useContext, useState, useCallback } from "react";
import { ViewMode } from "#/components/layout/Navigation";

interface NavigationContextType {
	activeView: ViewMode;
	setActiveView: (view: ViewMode) => void;
	navigateToSettings: () => void;
}

const NavigationContext = createContext<NavigationContextType | undefined>(
	undefined,
);

export const NavigationProvider: React.FC<{ children: React.ReactNode }> = ({
	children,
}) => {
	const [activeView, setActiveView] = useState<ViewMode>("dashboard");

	const navigateToSettings = useCallback(() => {
		setActiveView("settings");
	}, []);

	const value = {
		activeView,
		setActiveView,
		navigateToSettings,
	};

	return (
		<NavigationContext.Provider value={value}>
			{children}
		</NavigationContext.Provider>
	);
};

export const useNavigationContext = (): NavigationContextType => {
	const context = useContext(NavigationContext);
	if (context === undefined) {
		throw new Error(
			"useNavigationContext must be used within a NavigationProvider",
		);
	}
	return context;
};