// React.
import { Fragment } from 'react';

// Custom.
import {
	NavigationBar,
	NavigationButton,
	NavigationGroup
} from '@website/common/component/navigation';

/**
 * A landing page component.
 *
 * @returns The landing page.
 */
export function LandingPage(): JSX.Element {
	return (
		<Fragment>
			<NavigationBar>
				<NavigationGroup>
					<NavigationButton href='/'>Easy Tracker</NavigationButton>
				</NavigationGroup>
				<NavigationGroup>
					<NavigationButton href='/login'>Login</NavigationButton>
					<NavigationButton href='/signup'>Sign Up</NavigationButton>
				</NavigationGroup>
			</NavigationBar>
		</Fragment>
	);
}
