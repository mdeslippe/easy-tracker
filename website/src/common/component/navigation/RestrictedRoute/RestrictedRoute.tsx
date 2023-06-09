// React.
import { Fragment, ReactNode } from 'react';

// React router.
import { Navigate } from 'react-router';

/**
 * Properties for the {@link RestrictedRoute} component.
 */
export interface RestrictedRouteProps {
	/**
	 * If the user is permitted to access the route.
	 */
	permitted: boolean;

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
 * A restricted route component.
 *
 * @param props The component's properties.
 * @returns The restricted route.
 */
export function RestrictedRoute({
	permitted,
	redirectTo,
	children
}: RestrictedRouteProps): JSX.Element {
	// If the user is not permitted to access the restricted route, redirect them.
	if (!permitted) return <Navigate to={redirectTo} />;

	// If the user is permitted to access the restricted route, render the content.
	return <Fragment>{children}</Fragment>;
}
