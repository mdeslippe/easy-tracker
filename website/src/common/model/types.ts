/**
 * Details about a validation error.
 */
export type ValidationErrorDetails = {
	/**
	 * A code that identifies the category of error.
	 */
	code: string;

	/**
	 * An optional error message that can provide additional context.
	 */
	message?: string;

	/**
	 * Validation parameters that must be satisfied.
	 */
	params: {
		[param: string]: string | number | boolean;
	};
};

/**
 * The response data that will be returned from the server if validation errors occur.
 */
export type ValidationErrorResponseData = {
	/**
	 * Fields that contained validation errors.
	 */
	[field: string]: Array<ValidationErrorDetails>;
};
