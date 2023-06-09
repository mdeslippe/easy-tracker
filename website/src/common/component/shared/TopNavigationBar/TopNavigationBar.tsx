// Hooks.
import { useAuthenticationStatus } from '@website/feature/auth/hook';

// Custom.
import {
	NavigationBar,
	NavigationButton,
	NavigationGroup
} from '@website/common/component/navigation';

/**
 * A top navigation bar.
 *
 * @returns The top navigation bar.
 */
export function TopNavigationBar(): JSX.Element {
	// TODO: Handle an error occurring.
	// TODO: Add a skeleton while this is fetching.
	const { isLoading, isAuthenticated } = useAuthenticationStatus();

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
					<NavigationButton to='/logout'>Logout</NavigationButton>
				</NavigationGroup>
			)}
		</NavigationBar>
	);
}
