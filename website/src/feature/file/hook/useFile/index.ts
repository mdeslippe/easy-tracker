// React query.
import { QueryFunctionContext, useQuery, useQueryClient } from '@tanstack/react-query';

// Models.
import { GetFileResponseData } from '@website/feature/file/model';

// Services.
import { getFile } from '@website/feature/file/service';

/**
 * A function that can be used to get the underlying query key for the useFile hook.
 *
 * @param id The id of the file that the query key is being retrieved for.
 * @returns The query key for the useFile hook.
 */
export function getFileQueryKey(id: number): Array<string> {
	return ['file', String(id)];
}

/**
 * The data type that will be returned from the useFile hook.
 */
export type UseFileResult = {
	/**
	 * If data is currently being fetched.
	 */
	isLoading: boolean;

	/**
	 * If the data is being fetched for the first time.
	 */
	isInitialLoading: boolean;

	/**
	 * If an error occurred while fetching data.
	 */
	isError: boolean;

	/**
	 * If the user is authenticated.
	 */
	isAuthenticated: boolean;

	/**
	 * If a file was found with the id specified.
	 */
	isFound: boolean;

	/**
	 * The file's data.
	 */
	file: Exclude<GetFileResponseData, undefined> | null;

	/**
	 * A function that can be used to refetch the data.
	 */
	refetch: () => void;
};

/**
 * A hook to get file data.
 *
 * @param id The id of the file.
 * @returns The result.
 */
export function useFile(id: number): UseFileResult {
	const query = useQuery(
		getFileQueryKey(id),
		async (context: QueryFunctionContext<string[], unknown>) => {
			// Send the request.
			const result = await getFile(id, context.signal);

			// If an error occurred.
			if (result.status >= 500) throw Error();

			// Return the result.
			return result;
		},
		{
			cacheTime: Infinity,
			refetchOnWindowFocus: false,
			refetchOnReconnect: false,
			refetchOnMount: false
		}
	);

	return {
		isLoading: query.isLoading,
		isInitialLoading: query.isInitialLoading,
		isError: query.isError,
		isAuthenticated: query.data !== undefined && query.data.status !== 401,
		isFound: query.data !== undefined && query.data.status === 200,
		file: query.data?.data ?? null,
		refetch: async () => await query.refetch()
	};
}
