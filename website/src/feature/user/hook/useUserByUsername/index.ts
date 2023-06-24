// React query.
import { QueryFunctionContext, useQuery, useQueryClient } from '@tanstack/react-query';

// Models.
import { GetUserByUsernameResponseData } from '@website/feature/user/model';

// Services.
import { getUserByUsername } from '@website/feature/user/service';

/**
 * A function that can be used to get the underlying query key for the useUserByUsername hook.
 *
 * @param username The username of the user that the query key is being retrieved for.
 * @returns The query key for the useUserByUsername hook.
 */
export function getUserByUsernameQueryKey(username: string): Array<string> {
	return ['user_by_username', username];
}

/**
 * The data that will be returned from the useUserByUsername hook.
 */
export type UseUserByUsernameResult = {
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
	 * If a user was found with the username specified.
	 */
	isFound: boolean;

	/**
	 * The user's data.
	 */
	user: Exclude<GetUserByUsernameResponseData, undefined> | null;

	/**
	 * A function that can be used to refetch the data.
	 */
	refetch: () => void;
};

/**
 * A hook to get information about a user based on their username.
 *
 * @param username The username of the user.
 * @returns The result.
 */
export function useUserByUsername(username: string): UseUserByUsernameResult {
	const query = useQuery(
		getUserByUsernameQueryKey(username),
		async (context: QueryFunctionContext<string[], unknown>) => {
			// Send the request.
			const result = await getUserByUsername(username, context.signal);

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
		user: query.data?.data ?? null,
		refetch: async () => await query.refetch()
	};
}

/**
 * The data that will be returned from the useUserByUsernameInvalidator hook.
 */
export type UseUserByUsernameInvalidatorResult = () => Promise<void>;

/**
 * A hook to invalidate the useUserByUsername result.
 *
 * @param username The username of the user that the result is being invalidated for.
 * @returns The result.
 */
export function useUserByUsernameInvalidator(username: string): UseUserByUsernameInvalidatorResult {
	const queryClient = useQueryClient();
	return () => queryClient.invalidateQueries({ queryKey: getUserByUsernameQueryKey(username) });
}

/**
 * The data that will be returned from the useUserByUsernameResetter hook.
 */
export type UseUserByUsernameResetterResult = () => Promise<void>;

/**
 * A hook to reset the useUserByUsername result.
 *
 * @param username The username of the user that the result is being reset for.
 * @returns The result.
 */
export function useUserByUsernameResetter(username: string): UseUserByUsernameResetterResult {
	const queryClient = useQueryClient();
	return () => queryClient.resetQueries({ queryKey: getUserByUsernameQueryKey(username) });
}
