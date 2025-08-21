export default function ErrorList({ errors }: { errors: string[] }) {
	if (errors.length === 0) {
		return null;
	}

	return (
		<ul className="text-destructive text-sm list-disc pl-6 space-y-1">
			{errors.map((error, index) => (
				<li key={index}>{error}</li>
			))}
		</ul>
	);
}
