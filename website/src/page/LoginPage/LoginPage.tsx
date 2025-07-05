// React.
import { JSX, Fragment } from 'react';

// Custom.
import { LoginForm } from '@website/feature/auth/component';
import { TopNavigationBar } from '@website/common/component/shared';

// CSS.
import '@website/page/LoginPage/loginPage.css';

/**
 * A login page component.
 *
 * @returns The login page.
 */
export function LoginPage(): JSX.Element {
	return (
		<Fragment>
			<TopNavigationBar />
			<Main />
		</Fragment>
	);
}

/**
 * The main content for the login page.
 *
 * @returns The main content for the login page.
 */
function Main(): JSX.Element {
	return (
		<main id='login'>
			<div>
				<div className='login-welcome-container'>
					<img
						className='hide-medium no-select'
						src='/images/pictures/code.svg'
						alt=''
					/>
					<h1>Welcome back!</h1>
				</div>
				<div className='login-form-wrapper'>
					<LoginForm />
				</div>
			</div>
		</main>
	);
}
