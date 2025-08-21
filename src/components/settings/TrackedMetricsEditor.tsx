import { useState } from "react";
import { Input } from "#/components/ui/input";
import { Button } from "#/components/ui/button";
import ErrorList from "#/components/ui/errorList";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "#/components/ui/dialog";
import { DialogClose } from "@radix-ui/react-dialog";
import { metricFormSchema } from "./MetricConfiguration";

interface TrackedMetricsEditorProps {
	name: string;
	onMetricUpdate: ({
		newName,
		prevName,
	}: {
		newName: string;
		prevName: string;
	}) => void;
}

export const TrackedMetricsEditor = ({
	name,
	onMetricUpdate,
}: TrackedMetricsEditorProps) => {
	const [newMetricName, setNewMetricName] = useState(name);
	const [validationErrors, setValidationErrors] = useState<string[]>([]);
	const [isOpen, setIsOpen] = useState(false);
	const validateMetricName = (name: string): string[] => {
		const errors: string[] = [];

		// Zod validation
		const result = metricFormSchema.safeParse({ name });
		if (!result.success) {
			errors.push(...result.error.issues.map((err) => err.message));
		}

		return errors;
	};

	const handleSave = async () => {
		const trimmedName = newMetricName.trim();
		const errors = validateMetricName(trimmedName);

		if (errors.length > 0) {
			setValidationErrors(errors);
			return;
		}
		onMetricUpdate({
			newName: trimmedName,
			prevName: name,
		});
		setValidationErrors([]);
		setIsOpen(false);
	};

	const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		setNewMetricName(e.target.value);
		// Clear validation errors when user starts typing
		if (validationErrors.length > 0) {
			setValidationErrors([]);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={setIsOpen}>
			<DialogTrigger>
				<Button variant="default">Edit</Button>
			</DialogTrigger>
			<DialogContent>
				<DialogHeader>
					<DialogTitle>Edit Habit</DialogTitle>
					<DialogDescription>
						<Input
							type="text"
							value={newMetricName}
							onChange={handleInputChange}
							autoFocus
						/>
						<ErrorList errors={validationErrors} />
					</DialogDescription>
				</DialogHeader>
				<DialogFooter>
					<div className="flex items-center gap-3">
						<Button disabled={validationErrors.length > 0} onClick={handleSave}>
							Save
						</Button>
						<DialogClose asChild>
							<Button variant="secondary">Cancel</Button>
						</DialogClose>
					</div>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
};
