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
	return (
		<NavigationBar>
			<NavigationGroup>
				<NavigationButton to='/'>Easy Tracker</NavigationButton>
			</NavigationGroup>
			<NavigationGroup>
				<NavigationButton to='/login'>Login</NavigationButton>
				<NavigationButton to='/signup'>Sign Up</NavigationButton>
			</NavigationGroup>
		</NavigationBar>
	);
}
