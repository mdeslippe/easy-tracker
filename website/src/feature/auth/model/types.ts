// Models.
import { User } from '@website/feature/user/model';

/**
 * The request data for requests to login.
 */
export type LoginRequestData = {
	/**
	 * The user's username.
	 */
	username: string;

	/**
	 * The user's password.
	 */
	password: string;
};

/**
 * The response data for requests to login.
 */
export type LoginResponseData = User | undefined;

/**
 * The request data for requests to logout.
 */
export type LogoutRequestData = undefined;

/**
 * The response data for requests to logout.
 */
export type LogoutResponseData = undefined;

/**
 * The request data for requests to get the client's authentication status.
 */
export type AuthStatusRequestData = undefined;

/**
 * The response data for requests to get the client's authentication status.
 */
export type AuthStatusResponseData = boolean | undefined;
