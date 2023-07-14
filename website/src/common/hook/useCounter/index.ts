// React.
import { useReducer } from 'react';

/**
 * A hook that can be used to get integers in an ascending sequence.
 *
 * @param initialValue The initial value of the counter.
 * @returns A function that retrieves the next count.
 */
export function useCounter(initialValue: number = 0) {
	const [count, nextCount] = useReducer<(count: number) => number>(
		(count) => count + 1,
		initialValue
	);

	return () => {
		nextCount();
		return count;
	};
}
