// React query.
import { QueryFunctionContext, useQuery } from '@tanstack/react-query';

// Models.
import { User } from '@website/feature/user/model';
import { GetUserThatIsCurrentlyAuthenticatedResponseData } from '@website/feature/auth/model';

// Services.
import { getUserThatIsCurrentlyAuthenticated } from '@website/feature/auth/service';

// Utils.
import { HttpResponse } from '@website/utility/http';
import { QueryProvider } from '@website/utility/query';

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
	 * If the data was successfully fetched.
	 */
	isSuccess: boolean;

	/**
	 * If the user is authenticated or not.
	 */
	isAuthenticated: boolean;

	/**
	 * The user's data.
	 */
	data: User | null;

	/**
	 * A function that can be used to refetch the data.
	 */
	refetch: () => void;
};

/**
 * A query provider for the authenticated user query.
 */
export const AuthenticatedUserQueryProvider: QueryProvider<
	() => string[],
	HttpResponse<GetUserThatIsCurrentlyAuthenticatedResponseData>
> = {
	getKey: () => ['current_user'],
	getData: (context: QueryFunctionContext<string[], unknown>) =>
		getUserThatIsCurrentlyAuthenticated(undefined, context.signal)
};

/**
 * A hook to get the user that is currently authenticated.
 *
 * @returns The authenticated user hook result.
 */
export function useAuthenticatedUser(): UseAuthenticatedUserResult {
	const query = useQuery(
		AuthenticatedUserQueryProvider.getKey(),
		AuthenticatedUserQueryProvider.getData
	);

	return {
		isLoading: query.isLoading,
		isInitialLoading: query.isInitialLoading,
		isError: query.isError,
		isSuccess: query.isSuccess,
		isAuthenticated: query.data?.status === 200 ?? false,
		data: query.data?.data ?? null,
		refetch: async () => await query.refetch()
	};
}
