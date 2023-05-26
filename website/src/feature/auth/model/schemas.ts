// Zod.
import { z } from 'zod';

// Schemas.
import { UserSchema } from '@website/feature/user/model';

/**
 * A schema of the request data for requests to login.
 */
export const LoginRequestDataSchema = z.object({
	username: z.string(),
	password: z.string()
});

/**
 * A schema of the response data for request to login.
 */
export const LoginResponseDataSchema = z.union([UserSchema, z.undefined()]);

/**
 * A schema of the request data for requests to logout.
 */
export const LogoutRequestDataSchema = z.undefined();

/**
 * A schema of the response data for requests to logout.
 */
export const LogoutResponseDataSchema = z.undefined();

/**
 * A schema of the request data for requests to check the client's authentication status.
 */
export const AuthStatusRequestDataSchema = z.undefined();

/**
 * A schema of the response data for requests to check the client's authentication status.
 */
export const AuthStatusResponseDataSchema = z.union([z.boolean(), z.undefined()]);
