// React query.
import { QueryFunctionContext, useQuery, useQueryClient } from '@tanstack/react-query';

// Services.
import { isAuthenticated } from '@website/feature/auth/service';

/**
 * The data that will be returned from the useAuthenticationStatus hook.
 */
export type UseAuthenticationStatusResult = {
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
	 * A function that can be used to refetch the data.
	 */
	refetch: () => void;
};

/**
 * A hook to check if the the user is currently authenticated.
 *
 * @returns The authentication status hook result.
 */
export function useAuthenticationStatus(): UseAuthenticationStatusResult {
	const query = useQuery(
		['authentication_status'],
		async (context: QueryFunctionContext<string[], unknown>) => {
			// Send the request.
			const result = await isAuthenticated(undefined, context.signal);

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
		isAuthenticated: query.data?.data ?? false,
		refetch: async () => await query.refetch()
	};
}

/**
 * The data that will be returned from the useAuthenticationStatusInvalidator hook.
 */
export type UseAuthenticationStatusInvalidatorResult = () => void;

/**
 * A hook that can be used to invalidate the useAuthenticationStatus result.
 *
 * @returns A function that can b e used to invalidate the useAuthenticationStatus result.
 */
export function useAuthenticationStatusInvalidator(): UseAuthenticationStatusInvalidatorResult {
	const queryClient = useQueryClient();
	return () => queryClient.invalidateQueries({ queryKey: ['authentication_status'] });
}
