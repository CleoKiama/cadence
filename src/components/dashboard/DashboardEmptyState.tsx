import React from "react";
import { EmptyState } from "#/components/shared/EmptyState";
import { Button } from "#/components/ui/button";
import { useNavigationContext } from "#/contexts/NavigationContext";
import { formatDisplayDate } from "#/utils/dateUtils";
import { BarChart3, Settings, TrendingUp } from "lucide-react";

export const DashboardEmptyState: React.FC = () => {
	const { navigateToSettings } = useNavigationContext();

	return (
		<div className="space-y-8">
			{/* Welcome Section */}
			<div className="text-center py-8">
				<h1 className="text-3xl font-bold text-foreground mb-2">
					Welcome to Habitron! 🎉
				</h1>
				<p className="text-muted-foreground">
					{formatDisplayDate(new Date())} • Start your habit tracking journey today
				</p>
			</div>

			{/* Main Empty State */}
			<EmptyState
				icon={
					<div className="relative">
						<BarChart3 className="h-16 w-16 text-muted-foreground mx-auto mb-2" />
						<TrendingUp className="h-8 w-8 text-primary absolute -top-2 -right-2" />
					</div>
				}
				title="Ready to build amazing habits?"
				description="Create your first metric to start tracking your progress. Whether it's daily exercise, reading, coding, or any other habit - we'll help you stay consistent and see your growth over time."
				action={
					<div className="space-y-4">
						<Button onClick={navigateToSettings} size="lg" className="px-8">
							<Settings className="mr-2 h-5 w-5" />
							Add Your First Metric
						</Button>
						<div className="text-center space-y-2">
							<p className="text-sm text-muted-foreground">
								Popular metrics to get started:
							</p>
							<div className="flex flex-wrap justify-center gap-2 text-xs">
								<span className="px-2 py-1 bg-muted rounded-full">Daily Exercise</span>
								<span className="px-2 py-1 bg-muted rounded-full">Pages Read</span>
								<span className="px-2 py-1 bg-muted rounded-full">Code Commits</span>
								<span className="px-2 py-1 bg-muted rounded-full">Water Intake</span>
							</div>
						</div>
					</div>
				}
			/>
		</div>
	);
};