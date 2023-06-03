// React.
import { Fragment } from 'react';

// Custom.
import { SignUpForm } from '@website/feature/user/component';
import { TopNavigationBar } from '@website/common/component/shared';

// CSS.
import '@website/page/SignUpPage/signUpPage.css';

/**
 * A sign up page component.
 *
 * @returns The sign up page.
 */
export function SignUpPage(): JSX.Element {
	return (
		<Fragment>
			<TopNavigationBar />
			<Main />
		</Fragment>
	);
}

/**
 * The main content for the sign up page.
 *
 * @returns The main content for the sign up page.
 */
function Main(): JSX.Element {
	return (
		<main id='signup'>
			<div>
				<div className='signup-welcome-container'>
					<img
						className='hide-medium no-select'
						src='/images/pictures/code.svg'
						alt=''
					/>
					<h1>Let's get started!</h1>
				</div>
				<div className='signup-form-wrapper'>
					<SignUpForm />
				</div>
			</div>
		</main>
	);
}
