// Zod.
import { z } from 'zod';

/**
 * A user schema.
 */
export const UserSchema = z.object({
	/**
	 * The user's unique identifier.
	 */
	id: z.number(),

	/**
	 * The date and time the user's account was created at.
	 */
	account_created_at: z.coerce.date(),

	/**
	 * A url to the user's profile picture.
	 */
	profile_picture_url: z.string(),

	/**
	 * The user's username.
	 */
	username: z.string(),

	/**
	 * The user's email address.
	 */
	email: z.string(),

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
