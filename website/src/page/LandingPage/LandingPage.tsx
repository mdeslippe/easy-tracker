// React.
import { JSX, Fragment } from 'react';

// React router.
import { Link } from 'react-router-dom';

// Custom.
import { TopNavigationBar } from '@website/common/component/shared';

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
			<TopNavigationBar />
			<Main />
		</Fragment>
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
					<Link
						to='/signup'
						className='large-button primary-button'
					>
						Get Started
					</Link>
					<Link
						to='#information'
						className='large-button secondary-button'
					>
						Learn More
					</Link>
				</div>
			</section>
		</main>
	);
}
