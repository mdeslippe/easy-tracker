// React.
import { Fragment } from 'react';

// Custom.
import { MaxWidthContainer } from '@website/common/component/layout';
import {
	NavigationBar,
	NavigationButton,
	NavigationGroup
} from '@website/common/component/navigation';

// CSS.
import '@website/page/LandingPage/landingPage.css';

/**
 * A landing page component.
 *
 * @returns The landing page.
 */
export function LandingPage(): JSX.Element {
	return (
		<Fragment>
			<Header />
			<Main />
		</Fragment>
	);
}

/**
 * The header for the landing page.
 *
 * @returns The header for the landing page.
 */
function Header(): JSX.Element {
	return (
		<NavigationBar>
			<NavigationGroup>
				<NavigationButton href='/'>Easy Tracker</NavigationButton>
			</NavigationGroup>
			<NavigationGroup>
				<NavigationButton href='/login'>Login</NavigationButton>
				<NavigationButton href='/signup'>Sign Up</NavigationButton>
			</NavigationGroup>
		</NavigationBar>
	);
}

/**
 * The main content for the landing page.
 *
 * @returns The main content for the landing page.
 */
function Main(): JSX.Element {
	return (
		<main>
			<section id='hero'>
				<h1>Take Back Control</h1>
				<p>
					In an ever-increasingly digital world, detecting and mitigating service
					disruptions has become more important than ever before. With Easy Tracker, you
					can be confident that your digital services are fully operational.
				</p>
				<div>
					<a
						href='signup'
						className='large-button primary-button'
					>
						Get Started
					</a>
					<a
						href='#about'
						className='large-button secondary-button'
					>
						Learn More
					</a>
				</div>
			</section>
		</main>
	);
}
