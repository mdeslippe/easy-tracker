// React.
import { Fragment } from 'react';

// CSS.
import '@website/page/ErrorPage/errorPage.css';

/**
 * An error page component.
 *
 * @returns The error page.
 */
export function ErrorPage(): JSX.Element {
	return (
		<Fragment>
			<Main />
		</Fragment>
	);
}

/**
 * The main content for the error page.
 *
 * @returns The main content for the error page.
 */
function Main(): JSX.Element {
	return <main></main>;
}
