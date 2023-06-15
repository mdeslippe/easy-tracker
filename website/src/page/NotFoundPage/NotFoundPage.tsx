// React.
import { Fragment } from 'react';

// Custom.
import { TopNavigationBar } from '@website/common/component/shared';

// CSS.
import '@website/page/NotFoundPage/notFoundPage.css';

/**
 * A not found page component.
 *
 * @returns The not found page.
 */
export function NotFoundPage(): JSX.Element {
	return (
		<Fragment>
			<TopNavigationBar />
			<Main />
		</Fragment>
	);
}

/**
 * The main content for the not found page.
 *
 * @returns The main content for the not found page.
 */
function Main(): JSX.Element {
	return <main></main>;
}
