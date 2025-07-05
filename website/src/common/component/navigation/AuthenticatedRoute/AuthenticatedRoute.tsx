// React.
import { JSX, ComponentType } from 'react';

// React router.
import { Navigate } from 'react-router';

// Hooks.
import { useAuthenticationStatus } from '@website/feature/auth/hook';

// Custom.
import { LoadingOverlay } from '@website/common/component/display';
import { RestrictedRoute } from '@website/common/component/navigation';

/**
 * Properties for the {@link AuthenticatedRoute} component.
 */
export interface AuthenticatedRouteProps {
	/**
	 * The route users will be redirected to if they are not permitted to access the route.
	 */
	redirectTo: string;

	/**
	 * The component that will be rendered if the user is permitted to access the route.
	 */
	Component: ComponentType;
}

/**
 * An authenticated route component.
 *
 * @param props The component's properties.
 * @returns The authenticated route.
 */
export function AuthenticatedRoute({
	redirectTo,
	Component
}: AuthenticatedRouteProps): JSX.Element {
	const { isLoading, isInitialLoading, isError, isAuthenticated } = useAuthenticationStatus();

	// If an error occurred while loading the user's authentication status.
	if (isError) {
		return <Navigate to='/error' />;
	}

	// If the user's authentication status is loading for the first time.
	if (isLoading && isInitialLoading) {
		return <LoadingOverlay />;
	}

	return (
		<RestrictedRoute
			permitted={isLoading || isAuthenticated}
			redirectTo={redirectTo}
			Component={Component}
		/>
	);
}
