// React.
import { JSX, Fragment } from 'react';

// Custom.
import { TopNavigationBar } from '@website/common/component/shared';
import { UserInformationForm } from '@website/feature/user/component';
import { Card, CardBody, CardHeader } from '@website/common/component/display';

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
	return (
		<main id='user-settings'>
			<Card>
				<CardHeader>
					<h1>User Settings</h1>
				</CardHeader>
				<CardBody>
					<UserInformationForm />
				</CardBody>
			</Card>
		</main>
	);
}
