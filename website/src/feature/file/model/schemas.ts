// Zod.
import { z } from 'zod';

// Schemas.
import { ValidationErrorResponseDataSchema } from '@website/common/model';

// Constants.
import {
	FILE_DATA_MAX_LENGTH,
	FILE_DATA_MIN_LENGTH,
	FILE_ID_MAX_VALUE,
	FILE_ID_MIN_VALUE,
	FILE_MIME_TYPE_MAX_LENGTH,
	FILE_MIME_TYPE_MIN_LENGTH,
	FILE_NAME_MAX_LENGTH,
	FILE_NAME_MIN_LENGTH,
	FILE_USER_ID_MAX_VALUE,
	FILE_USER_ID_MIN_VALUE
} from '@website/feature/file/constant';

// Utils.
import {
	createInvalidMaximumLengthErrorMessage,
	createInvalidMaximumNumberErrorMessage,
	createInvalidMinimumLengthErrorMessage,
	createInvalidMinimumNumberErrorMessage
} from '@website/utility/validation';

/**
 * A file schema.
 */
export const FileSchema = z.object({
	id: z.number().int().min(FILE_ID_MIN_VALUE).max(FILE_ID_MAX_VALUE),
	userId: z.number().int().min(FILE_USER_ID_MIN_VALUE).max(FILE_USER_ID_MAX_VALUE),
	fileCreatedAt: z.coerce.date(),
	mimeType: z.string().min(FILE_MIME_TYPE_MIN_LENGTH).max(FILE_MIME_TYPE_MAX_LENGTH),
	name: z.string().min(FILE_NAME_MIN_LENGTH).max(FILE_NAME_MAX_LENGTH),
	data: z.number().array().min(FILE_DATA_MIN_LENGTH).max(FILE_DATA_MAX_LENGTH)
});

/**
 * A schema of the request data for requests to create a file.
 */
export const CreateFileRequestDataSchema = z.object({
	mimeType: z
		.string()
		.min(FILE_MIME_TYPE_MIN_LENGTH, {
			message: createInvalidMinimumLengthErrorMessage('mimeType', FILE_MIME_TYPE_MIN_LENGTH)
		})
		.max(FILE_MIME_TYPE_MAX_LENGTH, {
			message: createInvalidMaximumLengthErrorMessage('mimeType', FILE_MIME_TYPE_MAX_LENGTH)
		}),
	name: z
		.string()
		.min(FILE_NAME_MIN_LENGTH, {
			message: createInvalidMinimumLengthErrorMessage('name', FILE_NAME_MIN_LENGTH)
		})
		.max(FILE_NAME_MAX_LENGTH, {
			message: createInvalidMaximumLengthErrorMessage('name', FILE_NAME_MAX_LENGTH)
		}),
	data: z
		.number()
		.array()
		.min(FILE_DATA_MIN_LENGTH, {
			message: createInvalidMinimumLengthErrorMessage('data', FILE_DATA_MIN_LENGTH)
		})
		.max(FILE_DATA_MAX_LENGTH, {
			message: createInvalidMaximumLengthErrorMessage('data', FILE_DATA_MAX_LENGTH)
		})
});

/**
 * A schema of the response data for requests to create a file.
 */
export const CreateFileResponseDataSchema = z.union([
	FileSchema,
	ValidationErrorResponseDataSchema,
	z.undefined()
]);

/**
 * A schema of the request data for requests to get a file.
 */
export const GetFileRequestDataSchema = z
	.number()
	.int()
	.min(FILE_ID_MIN_VALUE, {
		message: createInvalidMinimumNumberErrorMessage('id', FILE_ID_MIN_VALUE)
	})
	.max(FILE_ID_MAX_VALUE, {
		message: createInvalidMaximumNumberErrorMessage('id', FILE_ID_MAX_VALUE)
	});

/**
 * A schema of the response data for requests to get a file.
 */
export const GetFileResponseDataSchema = z.union([FileSchema, z.undefined()]);

/**
 * A schema of the request data for requests to update a file.
 */
export const UpdateFileRequestDataSchema = z.object({
	id: z
		.number()
		.int()
		.min(FILE_ID_MIN_VALUE, {
			message: createInvalidMinimumNumberErrorMessage('id', FILE_ID_MIN_VALUE)
		})
		.max(FILE_ID_MAX_VALUE, {
			message: createInvalidMaximumNumberErrorMessage('id', FILE_ID_MAX_VALUE)
		}),
	mimeType: z
		.string()
		.min(FILE_MIME_TYPE_MIN_LENGTH, {
			message: createInvalidMinimumLengthErrorMessage('mimeType', FILE_MIME_TYPE_MIN_LENGTH)
		})
		.max(FILE_MIME_TYPE_MAX_LENGTH, {
			message: createInvalidMaximumLengthErrorMessage('mimeType', FILE_MIME_TYPE_MAX_LENGTH)
		})
		.optional(),
	name: z
		.string()
		.min(FILE_NAME_MIN_LENGTH, {
			message: createInvalidMinimumLengthErrorMessage('name', FILE_NAME_MIN_LENGTH)
		})
		.max(FILE_NAME_MAX_LENGTH, {
			message: createInvalidMaximumLengthErrorMessage('name', FILE_NAME_MAX_LENGTH)
		})
		.optional(),
	data: z
		.number()
		.array()
		.min(FILE_DATA_MIN_LENGTH, {
			message: createInvalidMinimumLengthErrorMessage('data', FILE_DATA_MIN_LENGTH)
		})
		.max(FILE_DATA_MAX_LENGTH, {
			message: createInvalidMaximumLengthErrorMessage('data', FILE_DATA_MAX_LENGTH)
		})
		.optional()
});

/**
 * A schema of the response data for requests to update a file.
 */
export const UpdateFileResponseDataSchema = z.union([
	FileSchema,
	ValidationErrorResponseDataSchema,
	z.undefined()
]);

/**
 * A schema of the request data for requests to delete a file.
 */
export const DeleteFileRequestDataSchema = z
	.number()
	.int()
	.min(FILE_ID_MIN_VALUE, {
		message: createInvalidMinimumNumberErrorMessage('id', FILE_ID_MIN_VALUE)
	})
	.max(FILE_ID_MAX_VALUE, {
		message: createInvalidMaximumNumberErrorMessage('id', FILE_ID_MAX_VALUE)
	});

/**
 * A schema of the response data for requests to delete a file.
 */
export const DeleteFileResponseDataSchema = z.undefined();
