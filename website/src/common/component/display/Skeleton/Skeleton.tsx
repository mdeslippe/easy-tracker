// React.
import { ComponentProps } from 'react';

// CSS.
import '@website/common/component/display/Skeleton/skeleton.css';

/**
 * Properties for the {@link Skeleton} component.
 */
export interface SkeletonProps extends Omit<ComponentProps<'div'>, 'aria-busy'> {}

/**
 * A skeleton component.
 *
 * @param props The component's properties.
 * @returns The skeleton component.
 */
export function Skeleton({ className, ...props }: SkeletonProps): JSX.Element {
	return (
		<div
			{...props}
			aria-busy='true'
			className={className ? `${className} skeleton` : 'skeleton'}
		/>
	);
}
