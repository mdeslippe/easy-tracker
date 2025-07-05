// React.
import { JSX, ChangeEvent, ComponentProps, ForwardedRef, forwardRef } from 'react';

// CSS.
import '@website/common/component/input/FileSelectionBox/fileSelectionBox.css';

/**
 * Properties for the {@link FileSelectionBoxWithRef} component.
 */
export interface FileSelectionBoxProps
	extends Omit<ComponentProps<'input'>, 'id' | 'type' | 'onChange'> {
	/**
	 * The file selection box's label.
	 */
	label: string;

	/**
	 * A name that uniquely identifies the file selection box.
	 */
	name: string;

	/**
	 * The file format types that the user is allow to select.
	 */
	accept: string;

	/**
	 * A function that will be invoked when the user selects a file.
	 *
	 * @param event The file selection event.
	 */
	onSelect: (event: ChangeEvent<HTMLInputElement>) => void;
}

/**
 * A file selection box component with a forwarded reference.
 *
 * @param props The component's properties.
 * @param ref The reference that will be forwarded.
 * @returns The file selection box.
 */
function FileSelectionBoxWithRef(
	{ label, name, accept, onSelect, className, ...props }: FileSelectionBoxProps,
	ref: ForwardedRef<HTMLInputElement>
): JSX.Element {
	return (
		<div className={className ? `${className} file-selection-box` : 'file-selection-box'}>
			<label htmlFor={name}>{label}</label>
			<input
				ref={ref}
				id={name}
				name={name}
				accept={accept}
				type='file'
				onChange={onSelect}
				{...props}
			/>
		</div>
	);
}

/**
 * A file selection box component.
 */
export const FileSelectionBox = forwardRef(FileSelectionBoxWithRef);
