// React query.
import { QueryFunctionContext, useQuery, useQueryClient } from '@tanstack/react-query';

// Models.
import { User } from '@website/feature/user/model';

// Services.
import { getUserThatIsCurrentlyAuthenticated } from '@website/feature/auth/service';

/**
 * A function that can be used to get the underlying query key for the use authenticated user hook.
 *
 * @returns The query key for the user authenticated user hook.
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
	 * The user's data.
	 */
	user: User | null;

	/**
	 * A function that can be used to refetch the data.
	 */
	refetch: () => void;
};

/**
 * A hook to get the user that is currently authenticated.
 *
 * @returns The authenticated user hook result.
 */
export function useAuthenticatedUser(): UseAuthenticatedUserResult {
	const query = useQuery(
		getAuthenticatedUserQueryKey(),
		async (context: QueryFunctionContext<string[], unknown>) => {
			// Send the request.
			const result = await getUserThatIsCurrentlyAuthenticated(undefined, context.signal);

			// If an error occurred.
			if (result.status >= 500) throw Error();

			// Return the result.
			return result;
		},
		{
			cacheTime: Infinity
		}
	);

	return {
		isLoading: query.isLoading,
		isInitialLoading: query.isInitialLoading,
		isError: query.isError,
		isAuthenticated: query.data?.status === 200 ?? false,
		user: query.data?.data ?? null,
		refetch: async () => await query.refetch()
	};
}

/**
 * The data that will be returned from the useAuthenticatedUserInvalidator hook.
 */
export type UseAuthenticatedUserInvalidatorResult = () => void;

/**
 * A hook to invalidate the useAuthenticatedUser result.
 *
 * @returns The authenticated user invalidator hook result.
 */
export function useAuthenticatedUserInvalidator(): UseAuthenticatedUserInvalidatorResult {
	const queryClient = useQueryClient();
	return () => queryClient.invalidateQueries({ queryKey: getAuthenticatedUserQueryKey() });
}
