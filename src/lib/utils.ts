import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

type Success<T> = {
	data: T;
	error: null;
};

type Failure<E> = {
	error: E;
	data: null;
};

type Result<T, E> = Success<T> | Failure<E>;

export const tryCatch = async <T, E>(promise: T): Promise<Result<T, E>> => {
	try {
		const data = await promise;
		return { data, error: null };
	} catch (error) {
		return { data: null, error: error as E };
	}
};
