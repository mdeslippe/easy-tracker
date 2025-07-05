// React.
import { JSX, ComponentProps } from 'react';

// CSS.
import '@website/common/component/layout/MaxWidthContainer/maxWidthContainer.css';

/**
 * Properties for the {@link MaxWidthContainer} component.
 */
export interface MaxWidthContainerProps extends ComponentProps<'div'> {}

/**
 * A max width container component.
 *
 * @param props The component's properties.
 * @returns The max width container.
 */
export function MaxWidthContainer({
	className,
	children,
	...props
}: MaxWidthContainerProps): JSX.Element {
	return (
		<div
			{...props}
			className={
				className ? `max-width-container-outer ${className}` : 'max-width-container-outer'
			}
		>
			<div className='max-width-container-inner'>{children}</div>
		</div>
	);
}
