// React.
import { ComponentProps } from 'react';

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
		<nav
			{...props}
			className={className ? `top-nav ${className}` : 'top-nav'}
		>
			{children}
		</nav>
	);
}
