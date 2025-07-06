// React.
import { JSX, ComponentType } from 'react';

// React router.
import { Navigate } from 'react-router';

// Hooks.
import { useAuthenticationStatus } from '@website/feature/auth/hook';

// Custom.
import { LoadingOverlay } from '@website/common/component/display';

/**
 * Properties for the {@link ConditionalAuthenticationRoute} component.
 */
export interface ConditionalAuthenticationRouteProps {
	/**
	 * The component that will be rendered if the user is authenticated.
	 */
	AuthenticatedComponent: ComponentType;

	/**
	 * The component that will be rendered if the user is not authenticated.
	 */
	UnauthenticatedComponent: ComponentType;
}

/**
 * A conditional authentication route component.
 *
 * @param props The component's properties.
 * @returns The conditional authentication route.
 */
export function ConditionalAuthenticationRoute({
	AuthenticatedComponent,
	UnauthenticatedComponent
}: ConditionalAuthenticationRouteProps): JSX.Element {
	const { isLoading, isInitialLoading, isError, isAuthenticated } = useAuthenticationStatus();

	// If an error occurred while loading the user's authentication status.
	if (isError) {
		return <Navigate to='/error' />;
	}

	// If the user's authentication status is loading for the first time.
	if (isLoading && isInitialLoading) {
		return <LoadingOverlay />;
	}

	return isAuthenticated ? <AuthenticatedComponent /> : <UnauthenticatedComponent />;
}
