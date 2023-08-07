// React.
import { ChangeEvent, ComponentProps } from 'react';

// CSS.
import '@website/common/component/input/FileSelectionBox/fileSelectionBox.css';

/**
 * Properties for the {@link FileSelectionBox} component.
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
	 * A function that will be invoked when the user selects an file.
	 *
	 * @param event The file selection event.
	 */
	onSelect: (event: ChangeEvent<HTMLInputElement>) => void;
}

/**
 * An file selection box component.
 *
 * @param props The component's properties.
 * @returns The file selection box.
 */
export function FileSelectionBox({
	label,
	name,
	accept,
	onSelect,
	...props
}: FileSelectionBoxProps): JSX.Element {
	return (
		<div className='file-selection-box'>
			<label htmlFor={name}>{label}</label>
			<input
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
