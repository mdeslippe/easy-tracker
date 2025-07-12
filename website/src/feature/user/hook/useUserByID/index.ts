// React query.
import { QueryFunctionContext, useQuery, useQueryClient } from '@tanstack/react-query';

// Models.
import { GetUserByIDResponseData } from '@website/feature/user/model';

// Services.
import { getUserByID } from '@website/feature/user/service';

/**
 * A function that can be used to get the underlying query key for the useUserByID hook.
 *
 * @param id The id of the user that the query key is being retrieved for.
 * @returns The query key for the useUserByID hook.
 */
export function getUserByIDQueryKey(id: number): Array<string> {
	return ['user_by_id', String(id)];
}

/**
 * The data that will be returned from the useUserByID hook.
 */
export type UseUserByIDResult = {
	/**
	 * If data is currently being fetched.
	 */
	isLoading: boolean;

	/**
	 * If an error occurred while fetching data.
	 */
	isError: boolean;

	/**
	 * If the user is authenticated.
	 */
	isAuthenticated: boolean;

	/**
	 * If a user was found with the id specified.
	 */
	isFound: boolean;

	/**
	 * The user's data.
	 */
	user: Exclude<GetUserByIDResponseData, undefined> | null;

	/**
	 * A function that can be used to refetch the data.
	 */
	refetch: () => void;
};

/**
 * A hook to get information about a user based on their id.
 *
 * @param id The id of the user.
 * @returns The result.
 */
export function useUserByID(id: number): UseUserByIDResult {
	const query = useQuery(
		{
			queryKey: getUserByIDQueryKey(id),
			queryFn: async (context: QueryFunctionContext) => {
				// Send the request.
				const result = await getUserByID(id, context.signal);

				// If an error occurred.
				if (result.status >= 500) throw Error();

				// Return the result.
				return result;
			},
			staleTime: Infinity,
			refetchOnWindowFocus: false,
			refetchOnReconnect: false,
			refetchOnMount: false
		}
	);

	return {
		isLoading: query.isLoading,
		isError: query.isError,
		isAuthenticated: query.data !== undefined && query.data.status !== 401,
		isFound: query.data !== undefined && query.data.status === 200,
		user: query.data?.data ?? null,
		refetch: async () => await query.refetch()
	};
}

/**
 * The data that will be returned from the useUserByIDInvalidator hook.
 */
export type UseUserByIDInvalidatorResult = () => Promise<void>;

/**
 * A hook to invalidate the useUserByID result.
 *
 * @param id The id of the user that the result is being invalidated for.
 * @returns The result.
 */
export function useUserByIDInvalidator(id: number): UseUserByIDInvalidatorResult {
	const queryClient = useQueryClient();
	return () => queryClient.invalidateQueries({ queryKey: getUserByIDQueryKey(id) });
}

/**
 * The data that will be returned from the useUserByIDResetter hook.
 */
export type UseUserByIDResetterResult = () => Promise<void>;

/**
 * A hook to reset the useUserByID result.
 *
 * @param id The id of the user that the result is being reset for.
 * @returns The result.
 */
export function useUserByIDResetter(id: number): UseUserByIDResetterResult {
	const queryClient = useQueryClient();
	return () => queryClient.resetQueries({ queryKey: getUserByIDQueryKey(id) });
}
