// React query.
import { QueryFunctionContext } from '@tanstack/query-core';

/**
 * A query provider type.
 */
export type QueryProvider<F extends Function, T> = {
	/**
	 * Get a query key.
	 *
	 * @returns
	 */
	getKey: F;

	/**
	 * A function that returns the query data.
	 *
	 * @param context The context the function is being executed in.
	 * @returns A promise to the data.
	 */
	getData: (context: QueryFunctionContext<string[], unknown>) => Promise<T>;
};
