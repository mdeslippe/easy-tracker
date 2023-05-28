// React.
import { ComponentProps } from 'react';

// CSS.
import '@website/common/component/navigation/NavigationGroup/navigationGroup.css';

/**
 * Properties for the {@link NavigationGroup} component.
 */
export interface NavigationGroupProps extends ComponentProps<'ul'> {}

/**
 * A navigation group component.
 *
 * @param props The component's properties.
 * @returns The navigation group.
 */
export function NavigationGroup({
	className,
	children,
	...props
}: NavigationGroupProps): JSX.Element {
	return (
		<ul
			{...props}
			className={className ? `nav-group ${className}` : 'nav-group'}
		>
			{children}
		</ul>
	);
}
