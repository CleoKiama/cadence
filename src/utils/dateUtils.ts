//TODO: use date fns

export const formatDate = (date: string | Date): string => {
	const dateObj = typeof date === "string" ? new Date(date) : date;
	return dateObj.toISOString().split("T")[0];
};

export const formatDisplayDate = (date: string | Date): string => {
	const dateObj = typeof date === "string" ? new Date(date) : date;
	return dateObj.toLocaleDateString("en-US", {
		year: "numeric",
		month: "short",
		day: "numeric",
	});
};

export const getDateRange = (days: number) => {
	const end = new Date();
	const start = new Date();
	start.setDate(end.getDate() - days + 1);

	return {
		start: formatDate(start),
		end: formatDate(end),
	};
};

export const generateDateRange = (
	startDate: string,
	endDate: string,
): string[] => {
	const start = new Date(startDate);
	const end = new Date(endDate);
	const dates: string[] = [];

	const current = new Date(start);
	while (current <= end) {
		dates.push(formatDate(current));
		current.setDate(current.getDate() + 1);
	}

	return dates;
};
