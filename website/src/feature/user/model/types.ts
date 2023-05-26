import { ValidationErrorResponse } from '@website/common/model';

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
	accountCreatedAt: Date;

	/**
	 * A url to the user's profile picture.
	 */
	profilePictureUrl: string;

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
	emailIsVerified: boolean;

	/**
	 * If the user is required to reset their password before they can login.
	 */
	passwordResetIsRequired: boolean;

	/**
	 * If the user's account has been locked.
	 */
	accountIsLocked: boolean;

	/**
	 * If the user's account has been banned.
	 */
	accountIsBanned: boolean;
};

/**
 * The request data for requests to create a user.
 */
export type CreateUserRequestData = {
	/**
	 * The user's username.
	 */
	username: string;

	/**
	 * The user's password.
	 */
	password: string;

	/**
	 * The user's email address.
	 */
	email: string;
};

/**
 * The response data for requests to create a user.
 */
export type CreateUserResponseData = User | ValidationErrorResponse | undefined;

/**
 * The request data for requests to get a user by id.
 */
export type GetUserByIDRequestData = number;

/**
 * The response data for requests to get a user by id.
 */
export type GetUserByIDResponseData =
	| {
			/**
			 * The user's unique identifier.
			 */
			id: number;

			/**
			 * The date and time the user's account was created at.
			 */
			accountCreatedAt: Date;

			/**
			 * A url to the user's profile picture.
			 */
			profilePictureUrl: string;

			/**
			 * The user's username.
			 */
			username: string;
	  }
	| undefined;

/**
 * The request data for requests to get a user by username.
 */
export type GetUserByUsernameRequestData = string;

/**
 * The response data for requests to get a user by username.
 */
export type GetUserByUsernameResponseData =
	| {
			/**
			 * The user's unique identifier.
			 */
			id: number;

			/**
			 * The date and time the user's account was created at.
			 */
			accountCreatedAt: Date;

			/**
			 * A url to the user's profile picture.
			 */
			profilePictureUrl: string;

			/**
			 * The user's username.
			 */
			username: string;
	  }
	| undefined;

/**
 * The request data for requests to get a user by email address.
 */
export type GetUserByEmailRequestData = string;

/**
 * The response data for requests to get a user by email address.
 */
export type GetUserByEmailResponseData =
	| {
			/**
			 * The user's unique identifier.
			 */
			id: number;

			/**
			 * The date and time the user's account was created at.
			 */
			accountCreatedAt: Date;

			/**
			 * A url to the user's profile picture.
			 */
			profilePictureUrl: string;

			/**
			 * The user's username.
			 */
			username: string;
	  }
	| undefined;

/**
 * The request data for requests to update a user.
 */
export type UpdateUserRequestData = {
	/**
	 * A url to the user's profile picture.
	 */
	profilePictureUrl?: string;

	/**
	 * The user's username.
	 */
	username?: string;

	/**
	 * The user's password.
	 */
	password?: string;

	/**
	 * The user's email address.
	 */
	email?: string;
};

/**
 * The response data for requests to update a user.
 */
export type UpdateUserResponseData = User | ValidationErrorResponse | undefined;

/**
 * The request data for requests to delete a user.
 */
export type DeleteUserRequestData = undefined;

/**
 * The response data for requests to delete a user.
 */
export type DeleteUserResponseData = undefined;
