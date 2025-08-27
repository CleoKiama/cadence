import React from "react";
import { Skeleton } from "#/components/ui/skeleton";
import { formatDisplayDate } from "#/utils/dateUtils";

export const DashboardLoadingState: React.FC = () => {
	return (
		<div className="space-y-8">
			{/* Welcome Section */}
			<div className="text-center py-8">
				<h1 className="text-3xl font-bold text-foreground mb-2">
					Welcome back to Habitron
				</h1>
				<p className="text-muted-foreground">
					{formatDisplayDate(new Date())} • Track your habits, build your future
				</p>
			</div>

			{/* Streak Counter Skeleton */}
			<Skeleton className="h-32 w-full rounded-lg" />

			{/* Metric Grid Skeleton */}
			<div>
				<Skeleton className="h-6 w-32 mb-6" />
				<div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
					{[...Array(3)].map((_, i) => (
						<Skeleton key={i} className="h-48 w-full rounded-lg" />
					))}
				</div>
			</div>

			{/* Recent Activity Skeleton */}
			<div>
				<Skeleton className="h-6 w-40 mb-6" />
				<div className="space-y-4">
					<Skeleton className="h-8 w-48" />
					<Skeleton className="h-64 w-full rounded-lg" />
				</div>
			</div>
		</div>
	);
};