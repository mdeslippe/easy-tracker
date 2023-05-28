// React.
import { ComponentProps } from 'react';

// Custom.
import { MaxWidthContainer } from '../../layout';

// CSS.
import '@website/common/component/navigation/NavigationBar/navigationBar.css';

/**
 * Properties for the {@link NavigationBar} component.
 */
export interface NavigationBarProps extends ComponentProps<'nav'> {}

/**
 * A navigation bar component.
 *
 * @param props The component's properties.
 * @returns The navigation bar.
 */
export function NavigationBar({ className, children, ...props }: NavigationBarProps): JSX.Element {
	return (
		<MaxWidthContainer className='top-nav-container'>
			<nav
				{...props}
				className={className ? `top-nav ${className}` : 'top-nav'}
			>
				{children}
			</nav>
		</MaxWidthContainer>
	);
}
