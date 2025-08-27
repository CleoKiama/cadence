import React from "react";

interface EmptyStateProps {
	icon?: React.ReactNode;
	title: string;
	description: string;
	action?: React.ReactNode;
}

export const EmptyState: React.FC<EmptyStateProps> = ({
	icon,
	title,
	description,
	action,
}) => {
	return (
		<div className="flex flex-col items-center justify-center text-center py-12 px-4">
			{icon && <div className="mb-4">{icon}</div>}
			<h3 className="text-lg font-semibold text-foreground mb-2">{title}</h3>
			<p className="text-muted-foreground mb-6 max-w-md">{description}</p>
			{action && <div>{action}</div>}
		</div>
	);
};