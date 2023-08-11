// React image crop.
import { PixelCrop } from 'react-image-crop';

/**
 * Crop an image.
 *
 * @param image The image.
 * @param crop The crop to perform.
 * @param rotate The amount of degrees the image should be rotated.
 * @param scale The factor the image should be scaled by.
 * @param type The output image format type (this will default to png if the type is not supported).
 * @returns A promise to the cropped image as a blob.
 */
export async function cropImage(
	image: HTMLImageElement,
	crop: PixelCrop,
	rotate: number,
	scale: number,
	type: string
): Promise<Blob> {
	return await new Promise<Blob>(
		(resolve: (value: Blob) => void, reject: (reason: Error) => void) => {
			// Create a canvas.
			const canvas = document.createElement('canvas');
			const context = canvas.getContext('2d');

			// If we could not get the context, we cannot proceed.
			if (context === null) {
				reject(new Error('No 2d canvas context'));
				return;
			}

			// Calculate values needed to perform the crop.
			const centerX = image.naturalWidth / 2;
			const centerY = image.naturalHeight / 2;

			const scaleX = image.naturalWidth / image.width;
			const scaleY = image.naturalHeight / image.height;

			const cropX = crop.x * scaleX;
			const cropY = crop.y * scaleY;

			const rotateRads = rotate * (Math.PI / 180);
			const pixelRatio = window.devicePixelRatio;

			// Prepare the canvas.
			canvas.width = Math.floor(crop.width * scaleX * pixelRatio);
			canvas.height = Math.floor(crop.height * scaleY * pixelRatio);
			context.scale(pixelRatio, pixelRatio);
			context.imageSmoothingQuality = 'high';

			// Create a save point for the canvas.
			context.save();

			// Move the crop origin to the canvas origin (0,0).
			context.translate(-cropX, -cropY);
			// Move the origin to the center of the original position.
			context.translate(centerX, centerY);
			// Rotate around the origin.
			context.rotate(rotateRads);
			// Scale the image.
			context.scale(scale, scale);
			// Move the center of the image to the origin (0,0).
			context.translate(-centerX, -centerY);
			// Draw the image.
			context.drawImage(
				image,
				0,
				0,
				image.naturalWidth,
				image.naturalHeight,
				0,
				0,
				image.naturalWidth,
				image.naturalHeight
			);

			// Restore the canvas to the save point.
			context.restore();

			// Export the canvas to a blob.
			canvas.toBlob((blob) => {
				if (blob === null) {
					reject(new Error('Failed to create blob'));
				} else {
					resolve(blob);
				}
			}, type);
		}
	);
}
