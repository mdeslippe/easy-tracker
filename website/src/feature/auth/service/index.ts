// Models.
import {
	AuthStatusRequestData,
	AuthStatusResponseData,
	AuthStatusResponseDataSchema,
	GetUserThatIsCurrentlyAuthenticatedRequestData,
	GetUserThatIsCurrentlyAuthenticatedResponseData,
	GetUserThatIsCurrentlyAuthenticatedResponseDataSchema,
	LoginRequestData,
	LoginResponseData,
	LoginResponseDataSchema,
	LogoutRequestData,
	LogoutResponseData,
	LogoutResponseDataSchema
} from '@website/feature/auth/model';

// Utils.
import { HttpResponse, sendGetRequest, sendPostRequest } from '@website/utility/http';

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
export async function login(data: LoginRequestData): Promise<HttpResponse<LoginResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendPostRequest('/auth/login', data, undefined);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await LoginResponseDataSchema.parseAsync(response.data)
	};
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
export async function logout(data: LogoutRequestData): Promise<HttpResponse<LogoutResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendPostRequest('/auth/logout', data, undefined);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await LogoutResponseDataSchema.parseAsync(response.data)
	};
}

/**
 * Send a request to the server to check if the client is authenticated.
 *
 * @param data The request data.
 * @param signal A signal that can be used to abort the request.
 * @returns A promise to the authentication status response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function isAuthenticated(
	data: AuthStatusRequestData,
	signal: AbortSignal | undefined = undefined
): Promise<HttpResponse<AuthStatusResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendGetRequest('/auth/status', data, signal);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await AuthStatusResponseDataSchema.parseAsync(response.data)
	};
}

/**
 * Send a request to the server to get information about the user that is currently authenticated.
 *
 * @param data The request data.
 * @param signal A signal that can be used to abort the request.
 * @returns A promise to the user that is currently authenticated response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function getUserThatIsCurrentlyAuthenticated(
	data: GetUserThatIsCurrentlyAuthenticatedRequestData,
	signal: AbortSignal | undefined = undefined
): Promise<HttpResponse<GetUserThatIsCurrentlyAuthenticatedResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendGetRequest('/auth/user', data, signal);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await GetUserThatIsCurrentlyAuthenticatedResponseDataSchema.parseAsync(response.data)
	};
}
