// React.
import { JSX, ComponentProps } from 'react';

// Custom.
import { CloseIcon, IconSize } from '@website/common/component/display';

// CSS.
import '@website/common/component/display/ErrorBox/errorBox.css';

/**
 * Properties for the {@link ErrorBox} component.
 */
export interface ErrorBoxProps extends Omit<ComponentProps<'div'>, 'role'> {
	/**
	 * The error message that will be displayed in the error box.
	 */
	message: string;

	/**
	 * A function that will be invoked when the user wants to close the error box.
	 */
	onClose: () => void;
}

/**
 * An error box component.
 *
 * @returns The error box.
 */
export function ErrorBox({ message, onClose, className, ...props }: ErrorBoxProps): JSX.Element {
	return (
		<div
			role='alert'
			className={className ? `${className} error-box` : 'error-box'}
			{...props}
		>
			<button
				onClick={onClose}
				type='button'
				title='Dismiss the alert'
			>
				<CloseIcon />
			</button>
			<span>{message}</span>
		</div>
	);
}
