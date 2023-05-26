// Zod.
import { z } from 'zod';

// Constants.
import {
	USER_EMAIL_MAX_LENGTH,
	USER_EMAIL_MIN_LENGTH,
	USER_ID_MIN_VALUE,
	USER_PROFILE_URL_MAX_LENGTH,
	USER_PROFILE_URL_MIN_LENGTH,
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
	id: z.number().int().min(USER_ID_MIN_VALUE).max(USER_ID_MIN_VALUE),

	/**
	 * The date and time the user's account was created at.
	 */
	account_created_at: z.coerce.date(),

	/**
	 * A url to the user's profile picture.
	 */
	profile_picture_url: z
		.string()
		.url()
		.min(USER_PROFILE_URL_MIN_LENGTH)
		.max(USER_PROFILE_URL_MAX_LENGTH),

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
	email_is_verified: z.boolean(),

	/**
	 * If the user is required to reset their password before they can login.
	 */
	password_reset_is_required: z.boolean(),

	/**
	 * If the user's account has been locked.
	 */
	account_is_locked: z.boolean(),

	/**
	 * If the user's account has been banned.
	 */
	account_is_banned: z.boolean()
});
