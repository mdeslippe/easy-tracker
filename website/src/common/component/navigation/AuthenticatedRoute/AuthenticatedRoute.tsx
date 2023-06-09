// React router.
import { PathRouteProps, Route } from 'react-router';

// Hooks.
import { useAuthenticationStatus } from '@website/feature/auth/hook';

// Custom.
import { RestrictedRoute } from '@website/common/component/navigation';

/**
 * Properties for the {@link AuthenticatedRoute} component.
 */
export interface AuthenticatedRouteProps extends PathRouteProps {
	/**
	 * The route users will be redirected to if they are not permitted to access the route.
	 */
	redirectTo: string;
}

/**
 * An authenticated route component.
 *
 * @param props The component's properties.
 * @returns The authenticated route.
 */
export function AuthenticatedRoute({ redirectTo, ...props }: AuthenticatedRouteProps): JSX.Element {
	const { isLoading, isAuthenticated } = useAuthenticationStatus();

	return (
		<RestrictedRoute
			permitted={isLoading || isAuthenticated}
			redirectTo={redirectTo}
		>
			<Route {...props} />
		</RestrictedRoute>
	);
}
