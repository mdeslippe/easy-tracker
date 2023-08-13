// React.
import { ComponentProps } from 'react';

// CSS.
import '@website/common/component/display/card/Card/card.css';

/**
 * Properties for the {@link Card} component.
 */
export interface CardProps extends ComponentProps<'div'> {}

/**
 * A card component.
 *
 * @param props The component's properties.
 * @returns The card component.
 */
export function Card({ className, children, ...props }: CardProps): JSX.Element {
	return (
		<div
			{...props}
			className={className ? `${className} card` : 'card'}
		>
			{children}
		</div>
	);
}
