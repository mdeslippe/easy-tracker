/**
 * A user type.
 */
export type User = {
	/**
	 * The user's unique identifier.
	 */
	id: number;

	/**
	 * The date and time the user's account was created at.
	 */
	account_created_at: Date;

	/**
	 * A url to the user's profile picture.
	 */
	profile_picture_url: string;

	/**
	 * The user's username.
	 */
	username: string;

	/**
	 * The user's email address.
	 */
	email: string;

	/**
	 * If the user has verified their email address.
	 */
	email_is_verified: boolean;

	/**
	 * If the user is required to reset their password before they can login.
	 */
	password_reset_is_required: boolean;

	/**
	 * If the user's account has been locked.
	 */
	account_is_locked: boolean;

	/**
	 * If the user's account has been banned.
	 */
	account_is_banned: boolean;
};
