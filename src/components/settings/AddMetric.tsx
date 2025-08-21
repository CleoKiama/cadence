import { useState } from "react";
import z from "zod";
import { Button } from "../ui/button";
import ErrorList from "../ui/errorList";
import { Input } from "../ui/input";
import { metricFormSchema, trackedMetricsShema } from "./MetricConfiguration";

export const AddMetricForm = ({
	metrics,
	onMetricUpdate,
}: {
	metrics: z.infer<typeof trackedMetricsShema>;
	onMetricUpdate: (newMetric: string) => void;
}) => {
	const [formValue, setFormValue] = useState("");
	const [validationErrors, setValidationErrors] = useState<string[]>([]);

	const validateMetricName = (name: string): string[] => {
		const errors: string[] = [];
		const result = metricFormSchema
			.superRefine((data, ctx) => {
				if (metrics.some((m) => m.name === data.name)) {
					ctx.addIssue({
						code: "custom",
						message: "Metric name already exists",
					});
				}
			})
			.safeParse({ name });
		if (!result.success) {
			errors.push(...result.error.issues.map((err) => err.message));
		}
		return errors;
	};

	const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		setFormValue(e.target.value);
		if (validationErrors.length > 0) {
			setValidationErrors([]);
		}
	};

	const handleSave = () => {
		const trimmedName = formValue.trim();
		const errors = validateMetricName(trimmedName);
		if (errors.length > 0) {
			setValidationErrors(errors);
			return;
		}
		onMetricUpdate(trimmedName);
	};

	return (
		<div className="space-y-4">
			<h3 className="text-lg font-semibold">Add New Metric</h3>
			<p className="text-sm text-muted-foreground">
				Add a new metric to track in your journal files
			</p>
			<Input type="text" value={formValue} onChange={handleInputChange} />
			<ErrorList errors={validationErrors} />
			<Button variant="default" onClick={handleSave}>
				Save
			</Button>
		</div>
	);
};
