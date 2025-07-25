// React.
import { JSX, ComponentProps, ForwardedRef, HTMLInputTypeAttribute, forwardRef } from 'react';

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
	ref: ForwardedRef<HTMLInputElement>
): JSX.Element {
	return (
		<div className='input-container'>
			<label
				className={required ? 'required' : undefined}
				htmlFor={name}
			>
				{label}
			</label>
			<div className='input-box'>
				<input
					ref={ref}
					id={name}
					name={name}
					{...props}
				/>
				<span className='input-underline' />
			</div>
			<span className='input-error-message'>{error}</span>
		</div>
	);
}

/**
 * An input field component.
 */
export const InputField = forwardRef(InputFieldWithRef);
