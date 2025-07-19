// React query.
import { QueryFunctionContext, useQuery, useQueryClient } from '@tanstack/react-query';

// Models.
import { User } from '@website/feature/user/model';

// Services.
import { getUserThatIsCurrentlyAuthenticated } from '@website/feature/auth/service';

/**
 * A function that can be used to get the underlying query key for the useAuthenticatedUser hook.
 *
 * @returns The query key for the useAuthenticatedUser hook.
 */
export function getAuthenticatedUserQueryKey(): Array<string> {
	return ['authentication_current_user'];
}

/**
 * The data that will be returned from the useAuthenticatedUser hook.
 */
export type UseAuthenticatedUserResult = {
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
	 * The user's data.
	 */
	user: User | null;

	/**
	 * A function that can be used to refetch the data.
	 */
	refetch: () => void;
};

/**
 * A hook to get information about the user that is currently authenticated.
 *
 * @returns The result.
 */
export function useAuthenticatedUser(): UseAuthenticatedUserResult {
	const query = useQuery(
		{
			queryKey: getAuthenticatedUserQueryKey(),
			queryFn: async (context: QueryFunctionContext) => {
				// Send the request.
				const result = await getUserThatIsCurrentlyAuthenticated(undefined, context.signal);

				// If an error occurred.
				if (result.status >= 500) throw Error();

				// Return the result.
				return result;
			},
			staleTime: Infinity
		}
	);

	return {
		isLoading: query.isLoading,
		isError: query.isError,
		isAuthenticated: query.data?.status === 200,
		user: query.data?.data ?? null,
		refetch: async () => await query.refetch()
	};
}

/**
 * The data that will be returned from the useAuthenticatedUserInvalidator hook.
 */
export type UseAuthenticatedUserInvalidatorResult = () => Promise<void>;

/**
 * A hook to invalidate the useAuthenticatedUser result.
 *
 * @returns The result.
 */
export function useAuthenticatedUserInvalidator(): UseAuthenticatedUserInvalidatorResult {
	const queryClient = useQueryClient();
	return () => queryClient.invalidateQueries({ queryKey: getAuthenticatedUserQueryKey() });
}

/**
 * The data that will be returned from the useAuthenticatedUserResetter hook.
 */
export type UseAuthenticatedUserResetterResult = () => Promise<void>;

/**
 * A hook to reset the useAuthenticatedUser result.
 *
 * @returns The result.
 */
export function useAuthenticatedUserResetter(): UseAuthenticatedUserResetterResult {
	const queryClient = useQueryClient();
	return () => queryClient.resetQueries({ queryKey: getAuthenticatedUserQueryKey() });
}
