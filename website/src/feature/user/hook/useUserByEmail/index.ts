// React query.
import { QueryFunctionContext, useQuery, useQueryClient } from '@tanstack/react-query';

// Models.
import { GetUserByEmailResponseData } from '@website/feature/user/model';

// Services.
import { getUserByEmail } from '@website/feature/user/service';

/**
 * A function that can be used to get the underlying query key for the useUserByEmail hook.
 *
 * @param email The email of the user that the query key is being retrieved for.
 * @returns The query key for the useUserByEmail hook.
 */
export function getUserByEmailQueryKey(email: string): Array<string> {
	return ['user_by_email', email];
}

/**
 * The data that will be returned from the useUserByEmail hook.
 */
export type UseUserByEmailResult = {
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
	 * If a user was found with the email specified.
	 */
	isFound: boolean;

	/**
	 * The user's data.
	 */
	user: Exclude<GetUserByEmailResponseData, undefined> | null;

	/**
	 * A function that can be used to refetch the data.
	 */
	refetch: () => void;
};

/**
 * A hook to get information about a user based on their email.
 *
 * @param email The email of the user.
 * @returns The result.
 */
export function useUserByEmail(email: string): UseUserByEmailResult {
	const query = useQuery(
		{
			queryKey: getUserByEmailQueryKey(email),
			queryFn: async (context: QueryFunctionContext) => {
				// Send the request.
				const result = await getUserByEmail(email, context.signal);

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
 * The data that will be returned from the useUserByEmailInvalidator hook.
 */
export type UseUserByEmailInvalidatorResult = () => Promise<void>;

/**
 * A hook to invalidate the useUserByEmail result.
 *
 * @param email The email of the user that the result is being invalidated for.
 * @returns The result.
 */
export function useUserByEmailInvalidator(email: string): UseUserByEmailInvalidatorResult {
	const queryClient = useQueryClient();
	return () => queryClient.invalidateQueries({ queryKey: getUserByEmailQueryKey(email) });
}

/**
 * The data that will be returned from the useUserByEmailResetter hook.
 */
export type UseUserByEmailResetterResult = () => Promise<void>;

/**
 * A hook to reset the useUserByEmail result.
 *
 * @param email The email of the user that the result is being reset for.
 * @returns The result.
 */
export function useUserByEmailResetter(email: string): UseUserByEmailResetterResult {
	const queryClient = useQueryClient();
	return () => queryClient.resetQueries({ queryKey: getUserByEmailQueryKey(email) });
}
