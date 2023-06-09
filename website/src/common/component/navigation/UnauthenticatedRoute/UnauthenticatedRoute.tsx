// React.
import { ReactNode } from 'react';

// Hooks.
import { useAuthenticationStatus } from '@website/feature/auth/hook';

// Custom.
import { RestrictedRoute } from '@website/common/component/navigation';

/**
 * Properties for the {@link AuthenticatedRoute} component.
 */
export interface UnauthenticatedRouteProps {
	/**
	 * The route users will be redirected to if they are not permitted to access the route.
	 */
	redirectTo: string;

	/**
	 * The content that will be rendered if the user is permitted to access the route.
	 */
	children: ReactNode;
}

/**
 * An unauthenticated route component.
 *
 * @param props The component's properties.
 * @returns The unauthenticated route.
 */
export function UnauthenticatedRoute({
	redirectTo,
	children
}: UnauthenticatedRouteProps): JSX.Element {
	// TODO: Handle an error occurring.
	const { isLoading, isAuthenticated } = useAuthenticationStatus();

	return (
		<RestrictedRoute
			permitted={isLoading || !isAuthenticated}
			redirectTo={redirectTo}
		>
			{children}
		</RestrictedRoute>
	);
}
