// React router.
import { Navigate } from 'react-router';

// Hooks.
import { useAuthenticationStatus } from '@website/feature/auth/hook';

// Custom.
import { UserMenu } from '@website/common/component/shared';
import { Skeleton } from '@website/common/component/display';
import {
	NavigationBar,
	NavigationButton,
	NavigationGroup
} from '@website/common/component/navigation';

// CSS.
import '@website/common/component/shared/TopNavigationBar/topNavigationBar.css';

/**
 * A top navigation bar.
 *
 * @returns The top navigation bar.
 */
export function TopNavigationBar(): JSX.Element {
	const { isLoading, isInitialLoading, isError, isAuthenticated } = useAuthenticationStatus();

	// If an error occurred while getting the user's authentication status.
	if (isError) {
		return <Navigate to='/error' />;
	}

	// If the navigation bar is loading for the first time, return a skeleton.
	if (isLoading && isInitialLoading) {
		return (
			<NavigationBar>
				<NavigationGroup>
					<NavigationButton to='/'>Easy Tracker</NavigationButton>
				</NavigationGroup>
				<NavigationGroup className='nav-group-skeleton'>
					<Skeleton className='nav-text-skeleton' />
					<Skeleton className='nav-text-skeleton' />
				</NavigationGroup>
			</NavigationBar>
		);
	}

	return (
		<NavigationBar>
			<NavigationGroup>
				<NavigationButton to='/'>Easy Tracker</NavigationButton>
			</NavigationGroup>
			{isLoading || !isAuthenticated ? (
				<NavigationGroup>
					<NavigationButton to='/login'>Login</NavigationButton>
					<NavigationButton to='/signup'>Sign Up</NavigationButton>
				</NavigationGroup>
			) : (
				<NavigationGroup>
					<UserMenu />
				</NavigationGroup>
			)}
		</NavigationBar>
	);
}
