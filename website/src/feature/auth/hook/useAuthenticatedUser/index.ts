// React query.
import { QueryFunctionContext, useQuery } from '@tanstack/react-query';

// Models.
import { User } from '@website/feature/user/model';

// Services.
import { getUserThatIsCurrentlyAuthenticated } from '@website/feature/auth/service';

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
	 * If the user is authenticated or not.
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
		['authenticated_user'],
		async (context: QueryFunctionContext<string[], unknown>) => {
			// Send the request.
			const result = await getUserThatIsCurrentlyAuthenticated(undefined, context.signal);

			// If an error occurred.
			if (result.status >= 500) throw Error();

			// Return the result.
			return result;
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
