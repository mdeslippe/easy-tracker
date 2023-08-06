// React.
import { ChangeEvent, ComponentProps, ForwardedRef, forwardRef } from 'react';

// CSS.
import '@website/common/component/input/ImageSelectionBox/imageSelectionBox.css';

/**
 * Properties for the {@link ImageSelectionBoxWithRef} component.
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
 * An image selection box component with a forwarded reference.
 *
 * @param props The component's properties.
 * @param ref The reference that will be forwarded.
 * @returns The image selection box.
 */
function ImageSelectionBoxWithRef(
	{ label, name, accept, onSelect, ...props }: ImageSelectionBoxProps,
	ref: ForwardedRef<HTMLInputElement>
): JSX.Element {
	return (
		<div className='image-selection-box'>
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
 * An image selection box component.
 */
export const ImageSelectionBox = forwardRef(ImageSelectionBoxWithRef);
