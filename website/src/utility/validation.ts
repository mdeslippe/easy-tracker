// Models.
import { ValidationErrorDetails } from '@website/common/model';

// Utils.
import { capitalizeFirstLetter } from '@website/utility/string';

/**
 * Create a generic error message.
 *
 * @param field The field that has a generic error.
 * @returns The generic error message that was created.
 */
export function createGenericErrorMessage(field: string) {
	return `${capitalizeFirstLetter(field)} is invalid.`;
}

/**
 * Create a required error message.
 *
 * @param field The field that is required.
 * @returns The required error message that was created.
 */
export function createRequiredErrorMessage(field: string) {
	return `${capitalizeFirstLetter(field)} is required.`;
}

/**
 * Create a must be unique error message.
 *
 * @param field The field that must be unique.
 * @returns The must be unique error message that was created.
 */
export function createMustBeUniqueErrorMessage(field: string) {
	return `${capitalizeFirstLetter(field)} is already in use.`;
}

/**
 * Create an invalid email error message.
 *
 * @returns The invalid email error message that was created.
 */
export function createInvalidEmailErrorMessage() {
	return 'Invalid email address.';
}

/**
 * Create an invalid url error message.
 *
 * @returns The invalid url error message that was created.
 */
export function createInvalidUrlErrorMessage() {
	return 'Invalid Url.';
}

/**
 * Create a control character error message.
 *
 * @returns The control character error message that was created.
 */
export function createInvalidControlCharacterErrorMessage() {
	return 'Control characters are not allowed.';
}

/**
 * Create a generic invalid length error message.
 *
 * @param field The field that has an invalid length.
 * @returns The generic invalid length error message that was created.
 */
export function createInvalidGenericLengthErrorMessage(field: string) {
	return `${capitalizeFirstLetter(field)} has an invalid length.`;
}

/**
 * Create an invalid minimum length error message.
 *
 * @param field The field that has an invalid length.
 * @param minimumLength The minimum length that is required.
 * @returns The invalid minimum length error message that was created.
 */
export function createInvalidMinimumLengthErrorMessage(field: string, minimumLength: number) {
	return `${capitalizeFirstLetter(field)} must be at least ${minimumLength} characters.`;
}

/**
 * Create an invalid maximum length error message.
 *
 * @param field The field that has an invalid length.
 * @param maximumLength The maximum length that cannot be exceeded.
 * @returns The invalid maximum length error message that was created.
 */
export function createInvalidMaximumLengthErrorMessage(field: string, maximumLength: number) {
	return `${capitalizeFirstLetter(field)} must not exceed ${maximumLength} characters.`;
}

/**
 * Create an invalid length range error message.
 *
 * @param field The field that has an invalid length.
 * @param minimumLength The minimum length that is required.
 * @param maximumLength The maximum length that cannot be exceeded.
 * @returns The invalid length range error message that was created.
 */
export function createInvalidLengthRangeErrorMessage(
	field: string,
	minimumLength: number,
	maximumLength: number
) {
	return `${capitalizeFirstLetter(
		field
	)} must be between ${minimumLength} and ${maximumLength} characters.`;
}

/**
 * Create a generic invalid number error message.
 *
 * @param field The filed that has the invalid number.
 * @returns The generic invalid number error message that was created.
 */
export function createGenericInvalidNumberErrorMessage(field: string) {
	return `${capitalizeFirstLetter(field)} has an invalid value.`;
}

/**
 * Create an invalid minimum number error message.
 *
 * @param field The filed that has the invalid number.
 * @param minimumNumber The minimum number that is required.
 * @returns The invalid minimum number error message that was created.
 */
export function createInvalidMinimumNumberErrorMessage(field: string, minimumNumber: number) {
	return `${capitalizeFirstLetter(field)} must be greater than or equal to ${minimumNumber}.`;
}

/**
 * Create an invalid maximum number error message.
 *
 * @param field The field that has the invalid number.
 * @param maximumNumber The maximum number that must not be exceeded.
 * @returns The invalid maximum number error message that was created.
 */
export function createInvalidMaximumNumberErrorMessage(field: string, maximumNumber: number) {
	return `${capitalizeFirstLetter(field)} must be less than or equal to ${maximumNumber}.`;
}

/**
 * Create an invalid number range error message.
 *
 * @param field The field that has the invalid number.
 * @param minimumNumber The minimum number that is required.
 * @param maximumNumber The maximum number that must not be exceeded.
 * @returns The invalid number range error message that was created.
 */
export function createInvalidNumberRangeErrorMessage(
	field: string,
	minimumNumber: number,
	maximumNumber: number
) {
	return `${capitalizeFirstLetter(
		field
	)} must be within the range ${minimumNumber}-${maximumNumber} (inclusive).`;
}

/**
 * A function that can be used to convert validation error details into an error message.
 *
 * This function currently has support for the following validation errors:
 * 1) required
 * 2) unique
 * 3) email
 * 4) url
 * 5) non_control_character
 * 6) length
 *
 * In the case that an unsupported error is detected, a generic error message will be returned.
 *
 * @param field The field that has the validation error.
 * @param error The validation error.
 * @returns A message representation of the validation error.
 */
export function convertValidationErrorToMessage(
	field: string,
	error: ValidationErrorDetails
): string {
	switch (error.code) {
		case 'required':
			return createRequiredErrorMessage(field);
		case 'unique':
			return createMustBeUniqueErrorMessage(field);
		case 'email':
			return createInvalidEmailErrorMessage();
		case 'url':
			return createInvalidUrlErrorMessage();
		case 'non_control_character':
			return createInvalidControlCharacterErrorMessage();
		case 'length':
			if (error.params.min !== undefined && error.params.max !== undefined)
				return createInvalidLengthRangeErrorMessage(
					field,
					Number(error.params.min),
					Number(error.params.max)
				);
			else if (error.params.min !== undefined)
				return createInvalidMinimumLengthErrorMessage(field, Number(error.params.min));
			else if (error.params.max !== undefined)
				return createInvalidMaximumLengthErrorMessage(field, Number(error.params.max));
			else return createInvalidGenericLengthErrorMessage(field);
		default:
			return createGenericErrorMessage(field);
	}
}
