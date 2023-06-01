// React.
import { ComponentProps, HTMLInputTypeAttribute, forwardRef } from 'react';

// CSS.
import '@website/common/component/input/InputField/inputField.css';

/**
 * Properties for the {@link InputFieldWithRef} component.
 */
export interface InputFieldProps extends Omit<ComponentProps<'input'>, 'id'> {
	/**
	 * The field's label.
	 */
	label: string;

	/**
	 * A name the uniquely identifies the field.
	 */
	name: string;

	/**
	 * The type of input field.
	 */
	type: HTMLInputTypeAttribute;

	/**
	 * The field's error message.
	 */
	error?: string | false;
}

/**
 * An input field component with a forwarded reference.
 *
 * @param props The component's properties.
 * @param ref The reference that will be forwarded.
 * @returns The input field.
 */
function InputFieldWithRef(
	{ label, name, error, required, ...props }: InputFieldProps,
	ref: React.ForwardedRef<HTMLInputElement>
): JSX.Element {
	const hasError: boolean = typeof error === 'string';

	return (
		<div className='form-input-container'>
			<label
				className={required ? 'required' : undefined}
				htmlFor={name}
			>
				{label}
			</label>
			<div className='input-box'>
				<input
					id={name}
					ref={ref}
					name={name}
					{...props}
				/>
				<span
					aria-hidden={true}
					className='input-underline'
				/>
			</div>
			<span
				aria-hidden={!hasError}
				className='form-error-message'
			>
				{error}
			</span>
		</div>
	);
}

/**
 * An input field component.
 */
export const InputField = forwardRef(InputFieldWithRef);
