// React.
import { JSX, Fragment } from 'react';

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
	return (
		<main id='not-found'>
			<div>
				<div className='not-found-message-container'>
					<h1>Not Found</h1>
					<p>The page you were looking for could not be found.</p>
				</div>
				<div className='not-found-picture-container'>
					<img
						src='/images/pictures/error-404.svg'
						alt=''
					/>
				</div>
			</div>
		</main>
	);
}
