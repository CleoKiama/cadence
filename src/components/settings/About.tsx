import { Card } from "../shared/Card";

export const About = () => {
	return (
		<Card>
			<div className="mb-6">
				<h3 className="text-lg font-semibold">About Habitron</h3>
				<p className="text-sm text-[var(--color-muted-foreground)]">
					A modern habit tracking application that syncs with your daily journal
				</p>
			</div>

			<div className="space-y-6">
				<div>
					<h4 className="font-medium mb-2">Features</h4>
					<ul className="text-sm text-[var(--color-muted-foreground)] space-y-1">
						<li>• Automatic data extraction from journal markdown files</li>
						<li>• Real-time file watching and synchronization</li>
						<li>• Beautiful analytics and progress tracking</li>
						<li>• Dark and light theme support</li>
						<li>• Privacy-focused: all data stays local</li>
					</ul>
				</div>

				<div>
					<h4 className="font-medium mb-2">Technology Stack</h4>
					<div className="flex flex-wrap gap-2">
						{["React", "TypeScript", "Tailwind CSS", "Tauri", "Rust"].map(
							(tech) => (
								<span
									key={tech}
									className="px-3 py-1 text-xs font-medium bg-[var(--color-muted)] text-[var(--color-muted-foreground)] rounded-full"
								>
									{tech}
								</span>
							),
						)}
					</div>
				</div>

				<div>
					<h4 className="font-medium mb-2">Version Information</h4>
					<div className="text-sm text-[var(--color-muted-foreground)]">
						<p>Version: 1.0.0</p>
						<p>Last Updated: {new Date().toLocaleDateString()}</p>
					</div>
				</div>

				<div className="pt-6 border-t border-[var(--color-border)]">
					<p className="text-sm text-[var(--color-muted-foreground)] text-center">
						Built with ❤ for habit tracking enthusiasts
					</p>
				</div>
			</div>
		</Card>
	);
};

