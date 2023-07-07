// Models.
import { ValidationErrorResponseData } from '@website/common/model';

/**
 * A file type.
 */
export type File = {
	/**
	 * The file's unique identifier.
	 */
	id: number;

	/**
	 * The unique identifier of the user that the file belongs to.
	 */
	userId: number;

	/**
	 * The date and time the file was created at.
	 */
	fileCreatedAt: Date;

	/**
	 * The file's mime type.
	 */
	mimeType: string;

	/**
	 * The file's name.
	 */
	name: string;

	/**
	 * The file's raw data.
	 */
	data: number[];
};

/**
 * The request data for requests to create a file.
 */
export type CreateFileRequestData = {
	/**
	 * The file's mime type.
	 */
	mimeType: string;

	/**
	 * The file's name.
	 */
	name: string;

	/**
	 * The file's raw data.
	 */
	data: number[];
};

/**
 * The response data for requests to create a file.
 */
export type CreateFileResponseData = File | ValidationErrorResponseData | undefined;

/**
 * The request data for requests to get a file.
 */
export type GetFileRequestData = number;

/**
 * The response data for requests to get a file.
 */
export type GetFileResponseData = File | undefined;

/**
 * The request data for requests to update a file.
 */
export type UpdateFileRequestData = {
	/**
	 * The file's unique identifier.
	 */
	id: number;

	/**
	 * The file's mime type.
	 */
	mimeType?: string;

	/**
	 * The file's name.
	 */
	name?: string;

	/**
	 * The file's raw data.
	 */
	data?: number[];
};

/**
 * The response data for requests to update a file.
 */
export type UpdateFileResponseData = File | ValidationErrorResponseData | undefined;

/**
 * The request data for requests to delete a file.
 */
export type DeleteFileRequestData = number;

/**
 * The response data for requests to delete a file.
 */
export type DeleteFileResponseData = undefined;
