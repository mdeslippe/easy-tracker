// Zod.
import { z } from 'zod';

// Schemas.
import { ValidationErrorResponseSchema } from '@website/common/model';

// Constants.
import {
	USER_EMAIL_MAX_LENGTH,
	USER_EMAIL_MIN_LENGTH,
	USER_ID_MAX_VALUE,
	USER_ID_MIN_VALUE,
	USER_PASSWORD_MAX_LENGTH,
	USER_PASSWORD_MIN_LENGTH,
	USER_PROFILE_PICTURE_URL_MAX_LENGTH,
	USER_PROFILE_PICTURE_URL_MIN_LENGTH,
	USER_USERNAME_MAX_LENGTH,
	USER_USERNAME_MIN_LENGTH
} from '@website/feature/user/constant';

/**
 * A user schema.
 */
export const UserSchema = z.object({
	/**
	 * The user's unique identifier.
	 */
	id: z.number().int().min(USER_ID_MIN_VALUE).max(USER_ID_MAX_VALUE),

	/**
	 * The date and time the user's account was created at.
	 */
	accountCreatedAt: z.coerce.date(),

	/**
	 * A url to the user's profile picture.
	 */
	profilePictureUrl: z
		.string()
		.url()
		.min(USER_PROFILE_PICTURE_URL_MIN_LENGTH)
		.max(USER_PROFILE_PICTURE_URL_MAX_LENGTH),

	/**
	 * The user's username.
	 */
	username: z.string().min(USER_USERNAME_MIN_LENGTH).max(USER_USERNAME_MAX_LENGTH),

	/**
	 * The user's email address.
	 */
	email: z.string().email().min(USER_EMAIL_MIN_LENGTH).max(USER_EMAIL_MAX_LENGTH),

	/**
	 * If the user has verified their email address.
	 */
	emailIsVerified: z.boolean(),

	/**
	 * If the user is required to reset their password before they can login.
	 */
	passwordResetIsRequired: z.boolean(),

	/**
	 * If the user's account has been locked.
	 */
	accountIsLocked: z.boolean(),

	/**
	 * If the user's account has been banned.
	 */
	accountIsBanned: z.boolean()
});

/**
 * A schema for request data for requests to create a user.
 */
export const CreateUserRequestDataSchema = z.object({
	username: z.string().min(USER_USERNAME_MIN_LENGTH).max(USER_USERNAME_MAX_LENGTH),
	password: z.string().min(USER_PASSWORD_MIN_LENGTH).max(USER_PASSWORD_MAX_LENGTH),
	email: z.string().email().min(USER_EMAIL_MIN_LENGTH).max(USER_EMAIL_MAX_LENGTH)
});

/**
 * A schema for response data for requests to create a user.
 */
export const CreateUserResponseDataSchema = z.union([
	UserSchema,
	ValidationErrorResponseSchema,
	z.undefined()
]);

/**
 * A schema for request data for requests to get a user by id.
 */
export const GetUserByIDRequestDataSchema = z
	.number()
	.int()
	.min(USER_ID_MIN_VALUE)
	.max(USER_ID_MAX_VALUE);

/**
 * A schema for response data for requests to get a user by id.
 */
export const GetUserByIDResponseDataSchema = z.union([
	z.object({
		id: z.number().int().min(USER_ID_MIN_VALUE).max(USER_ID_MAX_VALUE),
		accountCreatedAt: z.coerce.date(),
		profilePictureUrl: z
			.string()
			.url()
			.min(USER_PROFILE_PICTURE_URL_MIN_LENGTH)
			.max(USER_PROFILE_PICTURE_URL_MAX_LENGTH),
		username: z.string().min(USER_USERNAME_MIN_LENGTH).max(USER_USERNAME_MAX_LENGTH)
	}),
	z.undefined()
]);

/**
 * A schema for request data for requests to get a user by username.
 */
export const GetUserByUsernameRequestDataSchema = z
	.string()
	.min(USER_USERNAME_MIN_LENGTH)
	.max(USER_USERNAME_MAX_LENGTH);

/**
 * A schema for response data for requests to get a user by username.
 */
export const GetUserByUsernameResponseDataSchema = z.union([
	z.object({
		id: z.number().int().min(USER_ID_MIN_VALUE).max(USER_ID_MAX_VALUE),
		accountCreatedAt: z.coerce.date(),
		profilePictureUrl: z
			.string()
			.url()
			.min(USER_PROFILE_PICTURE_URL_MIN_LENGTH)
			.max(USER_PROFILE_PICTURE_URL_MAX_LENGTH),
		username: z.string().min(USER_USERNAME_MIN_LENGTH).max(USER_USERNAME_MAX_LENGTH)
	}),
	z.undefined()
]);

/**
 * A schema for request data for requests to get a user by email address.
 */
export const GetUserByEmailRequestDataSchema = z
	.string()
	.email()
	.min(USER_EMAIL_MIN_LENGTH)
	.max(USER_EMAIL_MAX_LENGTH);

/**
 * A schema for response data for requests to get a user by email address.
 */
export const GetUserByEmailResponseDataSchema = z.union([
	z.object({
		id: z.number().int().min(USER_ID_MIN_VALUE).max(USER_ID_MAX_VALUE),
		accountCreatedAt: z.coerce.date(),
		profilePictureUrl: z.string(),
		username: z.string().min(USER_USERNAME_MIN_LENGTH).max(USER_USERNAME_MAX_LENGTH)
	}),
	z.undefined()
]);

/**
 * A schema for request data for requests to update a user.
 */
export const UpdateUserRequestDataSchema = z.object({
	profilePictureUrl: z
		.string()
		.url()
		.min(USER_PROFILE_PICTURE_URL_MIN_LENGTH)
		.max(USER_PROFILE_PICTURE_URL_MAX_LENGTH)
		.optional(),
	username: z.string().min(USER_USERNAME_MIN_LENGTH).max(USER_USERNAME_MAX_LENGTH).optional(),
	password: z.string().min(USER_PASSWORD_MIN_LENGTH).max(USER_PASSWORD_MAX_LENGTH).optional(),
	email: z.string().email().min(USER_EMAIL_MIN_LENGTH).max(USER_EMAIL_MAX_LENGTH).optional()
});

/**
 * A schema for response data for requests to update a user.
 */
export const UpdateUserResponseDataSchema = z.union([
	UserSchema,
	ValidationErrorResponseSchema,
	z.undefined()
]);

/**
 * A schema for request data for requests to delete a user.
 */
export const DeleteUserRequestDataSchema = z.undefined();

/**
 * A schema for response data for requests to delete a user.
 */
export const DeleteUserResponseDataSchema = z.undefined();
