// React.
import { useState, ChangeEvent, useEffect, useRef } from 'react';

// Models.
import { File, FileSchema } from '@website/feature/file/model';

// Services.
import { createFile } from '@website/feature/file/service';

// Custom.
import { Modal } from '@website/common/component/layout';
import { ErrorBox } from '@website/common/component/display';
import { FileSelectionBox, ImageCropper } from '@website/common/component/input';

// Utils.
import { getFileUrl } from '@website/feature/file/utility';

// CSS.
import '@website/common/component/input/ImageSelector/imageSelector.css';

/**
 * Properties for the {@link ImageSelector} component.
 */
export interface ImageSelectorProps {
	/**
	 * An id that uniquely identifies the image selector.
	 */
	id: string;

	/**
	 * If the image selector is open.
	 */
	open: boolean;

	/**
	 * A title that will be displayed when the image selector is visible.
	 */
	title: string;

	/**
	 * A function that will be invoked when the user selects an image.
	 *
	 * @param url The url that the image is located at.
	 */
	onSelect: (url: string) => void;

	/**
	 * A function that will be invoked when the user wants to close the image selector.
	 */
	onClose: () => void;
}

/**
 * A function that can be used to handle uploading images.
 *
 * @param image The image that is being uploaded.
 * @returns A url to the image that was uploaded.
 */
async function handleImageUpload(image: Blob): Promise<string> {
	// Get the raw image data.
	const raw = await image.stream().getReader().read();

	// If the image does not contain any data.
	if (raw.value === undefined) {
		throw new Error('No image data');
	}

	// Save the image on the server.
	let createFileResponse = await createFile({
		data: Array.from(raw.value),
		mimeType: image.type,
		name: `picture.${image.type.replace('image/', '')}`
	});

	// Handle the file creation response.
	switch (createFileResponse.status) {
		case 200:
			// Parse the response.
			const file: File = await FileSchema.parseAsync(createFileResponse.data);

			// Return a url to the image that was uploaded.
			return getFileUrl(file.id);
		default:
			throw new Error('Failed to save image');
	}
}

/**
 * An image selector component.
 *
 * @param props The component's properties.
 * @returns The image selector.
 */
export function ImageSelector({
	id,
	open,
	title,
	onSelect,
	onClose
}: ImageSelectorProps): JSX.Element {
	const fileInputRef = useRef<HTMLInputElement | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [imageSource, setImageSource] = useState<string | null>(null);

	// Reset the state whenever the image selector opens or closes.
	useEffect(() => {
		setError(null);
		setImageSource(null);
	}, [open]);

	return (
		<Modal
			id={id}
			className='image-selector'
			open={open}
			title={title}
			onClose={onClose}
		>
			<div className='image-selector-content'>
				{error !== null && (
					<ErrorBox
						message={error}
						onClose={() => setError(null)}
					/>
				)}
				{imageSource === null ? (
					<FileSelectionBox
						ref={fileInputRef}
						className='image-selector-box'
						name={`${id}-selector-input`}
						label='Click here to select a picture'
						accept='image/png, image/jpeg, image/webp, image/gif, image/bmp'
						onSelect={(event: ChangeEvent<HTMLInputElement>) => {
							// If a file was selected.
							if (event.target.files !== null && event.target.files.length > 0) {
								// Create a file reader.
								const reader = new FileReader();

								// Add event listeners to the file reader.
								reader.addEventListener('load', () => {
									if (reader.result === null) {
										setError('Failed to load the picture.');
										setImageSource(null);
									} else {
										setError(null);
										setImageSource(reader.result.toString());
									}
								});

								reader.addEventListener('error', () => {
									if (fileInputRef.current !== null) {
										fileInputRef.current.value = '';
									}

									setError('Failed to load the picture.');
									setImageSource(null);
								});

								// Read the file that was selected.
								reader.readAsDataURL(event.target.files[0]);
							}
						}}
					/>
				) : (
					<ImageCropper
						className='image-selector-cropper'
						src={imageSource}
						aspectRatio={1}
						circular={true}
						exportType='image/webp'
						onComplete={async (image) => {
							try {
								const url = await handleImageUpload(image);
								onSelect(url);
							} catch (exception: unknown) {
								setError('Failed to upload the picture.');
							}
						}}
						onCancel={() => {
							setError(null);
							setImageSource(null);
						}}
						onError={() => {
							setError('Failed to process the picture.');
							setImageSource(null);
						}}
					/>
				)}
			</div>
		</Modal>
	);
}
