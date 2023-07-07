/**
 * An enumeration of the validation error types.
 */
export enum ValidationErrorType {
	/**
	 * A validation error type that indicates that the value is required.
	 */
	REQUIRED = 'required',

	/**
	 * A validation error type that indicates that the value must be unique.
	 */
	UNIQUE = 'unique',

	/**
	 * A validation error type that indicates that the value could not be found.
	 */
	NOT_FOUND = 'not_found',

	/**
	 * A validation error type that indicates that the value must be a valid email address.
	 */
	INVALID_EMAIL = 'email',

	/**
	 * A validation error type that indicates that the value must be a valid URL.
	 */
	INVALID_URL = 'url',

	/**
	 * A validation error type that indicates that the value must not contain control characters.
	 */
	INVALID_CHARACTER = 'non_control_character',

	/**
	 * A validation error type that indicates that the value must meet length constraints.
	 */
	INVALID_LENGTH = 'length',

	/**
	 * A validation error type that indicates that the value must be within a numerical range.
	 */
	INVALID_RANGE = 'range'
}
