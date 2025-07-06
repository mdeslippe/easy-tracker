// React.
import { JSX, Fragment } from 'react';

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
	return (
		<main id='error'>
			<div>
				<div className='error-message-container'>
					<h1>Oops...</h1>
					<p>
						It looks like an unexpected error has occurred, please try again later. We
						apologize for any inconvenience.
					</p>
				</div>
				<div className='error-picture-container'>
					<img
						src='/images/pictures/error-500.svg'
						alt=''
					/>
				</div>
			</div>
		</main>
	);
}
