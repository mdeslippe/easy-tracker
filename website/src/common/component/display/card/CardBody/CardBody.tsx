// React.
import { JSX, ComponentProps } from 'react';

// CSS.
import '@website/common/component/display/card/CardBody/cardBody.css';

/**
 * Properties for the {@link CardBody} component.
 */
export interface CardBodyProps extends ComponentProps<'div'> {}

/**
 * A card body component.
 *
 * @param props The component's properties.
 * @returns The card body.
 */
export function CardBody({ className, children, ...props }: CardBodyProps): JSX.Element {
	return (
		<div
			{...props}
			className={className ? `${className} card-body` : 'card-body'}
		>
			{children}
		</div>
	);
}
