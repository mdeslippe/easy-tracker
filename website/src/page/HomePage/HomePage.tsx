// React.
import { JSX, Fragment } from 'react';

// Custom.
import { TopNavigationBar } from '@website/common/component/shared';

// CSS.
import '@website/page/HomePage/homePage.css';

/**
 * A home page component.
 *
 * @returns The home page.
 */
export function HomePage(): JSX.Element {
	return (
		<Fragment>
			<TopNavigationBar />
			<Main />
		</Fragment>
	);
}

/**
 * The main content for the home page.
 *
 * @returns The main content for the home page.
 */
function Main(): JSX.Element {
	return <main></main>;
}
