// React.
import { JSX, ComponentProps } from 'react';

// CSS.
import '@website/common/component/display/card/CardHeader/cardHeader.css';

/**
 * Properties for the {@link CardHeader} component.
 */
export interface CardHeaderProps extends ComponentProps<'div'> {}

/**
 * A card header component.
 *
 * @param props The component's properties.
 * @returns The card header.
 */
export function CardHeader({ className, children, ...props }: CardHeaderProps): JSX.Element {
	return (
		<div
			{...props}
			className={className ? `${className} card-header` : 'card-header'}
		>
			{children}
		</div>
	);
}
