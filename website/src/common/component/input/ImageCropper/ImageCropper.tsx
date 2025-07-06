// React.
import { JSX, ChangeEvent, ComponentProps, useEffect, useRef, useState } from 'react';

// React image crop.
import ReactCrop, {
	Crop,
	PixelCrop,
	centerCrop,
	makeAspectCrop,
	convertToPixelCrop
} from 'react-image-crop';

// Utils.
import { cropImage } from '@website/utility/image';

// CSS.
import '@website/common/component/input/ImageCropper/imageCropper.css';

/**
 * Properties for the {@link ImageCropper} component.
 */
export interface ImageCropperProps extends ComponentProps<'div'> {
	/**
	 * The source of the image that is being cropped.
	 */
	src: string;

	/**
	 * The image type that the crop will be exported as.
	 * If the type specified is not supported, this will default to png.
	 */
	exportType: string;

	/**
	 * The aspect ratio of the crop.
	 */
	aspectRatio: number;

	/**
	 * If the crop is circular.
	 */
	circular: boolean;

	/**
	 * A function that will be invoked when the user confirms the crop.
	 *
	 * @param crop The raw crop data.
	 */
	onComplete: (crop: Blob) => void;

	/**
	 * A function that will be invoked when the user wants to cancel the crop.
	 */
	onCancel: () => void;

	/**
	 * A function that will be invoked if an error occurs.
	 *
	 * @param error The error that occurred.
	 */
	onError: (error: unknown) => void;
}

/**
 * An image cropper component.
 *
 * @param props The component's properties.
 * @returns The image cropper.
 */
export function ImageCropper({
	className,
	src,
	exportType,
	aspectRatio,
	circular,
	onComplete,
	onCancel,
	onError,
	...props
}: ImageCropperProps): JSX.Element {
	const imageRef = useRef<HTMLImageElement | null>(null);
	const [crop, setCrop] = useState<Crop | undefined>(undefined);
	const [completedCrop, setCompletedCrop] = useState<PixelCrop | undefined>(undefined);

	// If the image element displayed on the screen resizes, the calculations when performing the
	// crop will be incorrect. To avoid this issue, we must update the completed crop when the image
	// resizes.
	useEffect(() => {
		if (imageRef.current === null || crop === undefined) {
			return;
		}

		const resizeObserver = new ResizeObserver(() => {
			setCrop((crop) => {
				if (imageRef.current === null || crop === undefined) {
					setCompletedCrop(undefined);
					return undefined;
				} else {
					setCompletedCrop(
						convertToPixelCrop(crop, imageRef.current.width, imageRef.current.height)
					);
					return crop;
				}
			});
		});

		resizeObserver.observe(imageRef.current);
		return () => resizeObserver.disconnect();
	}, [imageRef.current, crop === undefined]);

	return (
		<div
			{...props}
			className={
				className ? `${className} image-cropper-container` : 'image-cropper-container'
			}
		>
			<ReactCrop
				crop={crop}
				onChange={(_, percentCrop) => setCrop(percentCrop)}
				onComplete={(crop) => setCompletedCrop(crop)}
				aspect={aspectRatio}
				circularCrop={circular}
			>
				<img
					ref={imageRef}
					alt='Crop me'
					src={src}
					onError={onError}
					onLoad={(event: ChangeEvent<HTMLImageElement>) => {
						// Get the width and height of the image.
						const { naturalWidth, naturalHeight } = event.currentTarget;

						// Crop create a default crop for the image.
						const crop = centerCrop(
							makeAspectCrop(
								{
									unit: '%',
									width: 80
								},
								aspectRatio,
								naturalWidth,
								naturalHeight
							),
							naturalWidth,
							naturalHeight
						);

						// Update the crop state.
						setCrop(crop);
					}}
				/>
			</ReactCrop>
			<div className='image-cropper-button-container'>
				<button
					className='primary-button small-button'
					type='button'
					disabled={imageRef.current === null || completedCrop === undefined}
					onClick={async () => {
						try {
							// If a crop has not been selected.
							if (imageRef.current === null || completedCrop === undefined) {
								return;
							}

							// Crop the image.
							const result: Blob = await cropImage(
								imageRef.current,
								completedCrop,
								0,
								1,
								exportType
							);

							// Return the result.
							onComplete(result);
						} catch (exception: unknown) {
							onError(exception);
						}
					}}
				>
					Confirm
				</button>
				<button
					className='secondary-button small-button'
					type='button'
					onClick={onCancel}
				>
					Cancel
				</button>
			</div>
		</div>
	);
}
