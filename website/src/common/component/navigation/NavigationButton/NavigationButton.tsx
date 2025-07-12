// React.
import { JSX } from 'react';

// React router.
import { NavLink, NavLinkProps } from 'react-router';

// CSS.
import '@website/common/component/navigation/NavigationButton/navigationButton.css';

/**
 * Properties for the {@link NavigationButton} component.
 */
export interface NavigationButtonProps extends NavLinkProps {}

/**
 * A navigation button component.
 *
 * @param props The component's properties.
 * @returns The navigation button.
 */
export function NavigationButton({ children, ...props }: NavigationButtonProps): JSX.Element {
	return (
		<li className='nav-button'>
			<NavLink {...props}>{children}</NavLink>
		</li>
	);
}
