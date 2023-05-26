// Models.
import {
	AuthStatusRequestData,
	AuthStatusResponseData,
	AuthStatusResponseDataSchema,
	LoginRequestData,
	LoginResponseData,
	LoginResponseDataSchema,
	LogoutRequestData,
	LogoutResponseData,
	LogoutResponseDataSchema
} from '@website/feature/auth/model';

// Utils.
import { sendGetRequest, sendPostRequest } from '@website/utility/http';

/**
 * Send a request to the server to authenticate a user.
 *
 * @param data The request data.
 * @returns A promise to the login response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function login(data: LoginRequestData): Promise<LoginResponseData> {
	// Send the request.
	const response = await sendPostRequest('/auth/login', data, undefined);

	// Parse the response body and return the result.
	return await LoginResponseDataSchema.parseAsync(response.data);
}

/**
 * Send a request to the server to unauthenticate a user.
 *
 * @param data The request data.
 * @returns A promise to the logout response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function logout(data: LogoutRequestData): Promise<LogoutResponseData> {
	// Send the request.
	const response = await sendPostRequest('/auth/logout', data, undefined);

	// Parse the response body and return the result.
	return await LogoutResponseDataSchema.parseAsync(response.data);
}

/**
 * Send a request to the server to check if the client is authenticated.
 *
 * @param data The request data.
 * @returns A promise to the authentication status response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function isAuthenticated(
	data: AuthStatusRequestData
): Promise<AuthStatusResponseData> {
	// Send the request.
	const response = await sendGetRequest('/auth/status', data, undefined);

	// Parse the response body and return the result.
	return await AuthStatusResponseDataSchema.parseAsync(response.data);
}
