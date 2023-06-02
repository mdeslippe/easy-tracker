// Zod.
import { z } from 'zod';

/**
 * A validation error schema.
 */
export const ValidationErrorDetailsSchema = z.object({
	/**
	 * A code that identifies the category of error.
	 */
	code: z.string(),

	/**
	 * An optional error message that can provide additional context.
	 */
	message: z.string().nullable().optional(),

	/**
	 * Validation parameters that must be satisfied.
	 */
	params: z.record(z.union([z.string(), z.number(), z.boolean()]))
});

/**
 * A schema of the response data that will be returned from the server if validation errors occur.
 */
export const ValidationErrorResponseDataSchema = z.record(
	/**
	 * Fields that contained validation errors.
	 */
	z.array(ValidationErrorDetailsSchema)
);
