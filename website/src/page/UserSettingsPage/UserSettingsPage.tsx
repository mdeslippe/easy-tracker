// React.
import { Fragment } from 'react';

// Custom.
import { TopNavigationBar } from '@website/common/component/shared';

// CSS.
import '@website/page/UserSettingsPage/userSettingsPage.css';

/**
 * A user settings page component.
 *
 * @returns The user settings page.
 */
export function UserSettingsPage(): JSX.Element {
	return (
		<Fragment>
			<TopNavigationBar />
			<Main />
		</Fragment>
	);
}

/**
 * The main content for the user settings page.
 *
 * @returns The main content for the user settings page.
 */
function Main(): JSX.Element {
	return <main></main>;
}
