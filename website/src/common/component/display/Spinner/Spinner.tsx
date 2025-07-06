// React.
import { JSX, ComponentProps } from 'react';

// CSS.
import '@website/common/component/display/Spinner/spinner.css';

/**
 * Properties for the {@link Spinner} component.
 */
export interface SpinnerProps extends ComponentProps<'span'> {}

/**
 * A spinner component.
 *
 * @returns The spinner.
 */
export function Spinner({ className, ...props }: SpinnerProps): JSX.Element {
	return (
		<span
			{...props}
			className={className ? `${className} spinner` : 'spinner'}
		/>
	);
}
