// React.
import { ComponentProps } from 'react';

// CSS.
import '@website/common/component/navigation/NavigationButton/navigationButton.css';

/**
 * Properties for the {@link NavigationButton} component.
 */
export interface NavigationButtonProps extends ComponentProps<'a'> {}

/**
 * A navigation button component.
 *
 * @param props The component's properties.
 * @returns The navigation button.
 */
export function NavigationButton({ children, ...props }: NavigationButtonProps): JSX.Element {
	return (
		<li className='nav-button'>
			<a {...props}>{children}</a>
		</li>
	);
}
