// React.
import { ChangeEvent, ComponentProps } from 'react';

// CSS.
import '@website/common/component/input/ImageSelectionBox/imageSelectionBox.css';

/**
 * Properties for the {@link ImageSelectionBox} component.
 */
export interface ImageSelectionBoxProps
	extends Omit<ComponentProps<'input'>, 'id' | 'type' | 'onChange'> {
	/**
	 * The image selection box's label.
	 */
	label: string;

	/**
	 * A name that uniquely identifies the image selection box.
	 */
	name: string;

	/**
	 * The file format types that the user is allow to select.
	 */
	accept: string;

	/**
	 * A function that will be invoked when the user selects an image.
	 *
	 * @param event The image selection event.
	 */
	onSelect: (event: ChangeEvent<HTMLInputElement>) => void;
}

/**
 * An image selection box component.
 *
 * @param props The component's properties.
 * @returns The image selection box.
 */
export function ImageSelectionBox({
	label,
	name,
	accept,
	onSelect,
	...props
}: ImageSelectionBoxProps): JSX.Element {
	return (
		<div className='image-selection-box'>
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
