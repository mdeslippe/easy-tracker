// Models.
import {
	CreateUserRequestData,
	CreateUserResponseData,
	CreateUserResponseDataSchema,
	DeleteUserRequestData,
	DeleteUserResponseData,
	DeleteUserResponseDataSchema,
	GetUserByEmailRequestData,
	GetUserByEmailResponseData,
	GetUserByEmailResponseDataSchema,
	GetUserByIDRequestData,
	GetUserByIDResponseData,
	GetUserByIDResponseDataSchema,
	GetUserByUsernameRequestData,
	GetUserByUsernameResponseData,
	GetUserByUsernameResponseDataSchema,
	UpdateUserRequestData,
	UpdateUserResponseData,
	UpdateUserResponseDataSchema
} from '@website/feature/user/model';

// Utils.
import {
	sendGetRequest,
	sendPostRequest,
	sendPatchRequest,
	sendDeleteRequest,
	HttpResponse
} from '@website/utility/http';

/**
 * Send a request to the server to create a new user.
 *
 * @param data The request data.
 * @returns A promise to the create user response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function createUser(
	data: CreateUserRequestData
): Promise<HttpResponse<CreateUserResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendPostRequest('/users', data, undefined);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await CreateUserResponseDataSchema.parseAsync(response.data)
	};
}

/**
 * Send a request to the server to get a user by their id.
 *
 * @param data The request data.
 * @param signal A signal that can be used to abort the request.
 * @returns A promise to the get user by id response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function getUserByID(
	data: GetUserByIDRequestData,
	signal: AbortSignal | undefined = undefined
): Promise<HttpResponse<GetUserByIDResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendGetRequest(
		`/users/id/${data}`,
		undefined,
		signal
	);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await GetUserByIDResponseDataSchema.parseAsync(response.data)
	};
}

/**
 * Send a request to the server to get a user by their username.
 *
 * @param data The request data.
 * @param signal A signal that can be used to abort the request.
 * @returns A promise to the get user by username response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function getUserByUsername(
	data: GetUserByUsernameRequestData,
	signal: AbortSignal | undefined = undefined
): Promise<HttpResponse<GetUserByUsernameResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendGetRequest(
		`/users/username/${data}`,
		undefined,
		signal
	);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await GetUserByUsernameResponseDataSchema.parseAsync(response.data)
	};
}

/**
 * Send a request to the server to get a user by their email address.
 *
 * @param data The request data.
 * @param signal A signal that can be used to abort the request.
 * @returns A promise to the get user by email response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function getUserByEmail(
	data: GetUserByEmailRequestData,
	signal: AbortSignal | undefined = undefined
): Promise<HttpResponse<GetUserByEmailResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendGetRequest(
		`/users/email/${data}`,
		undefined,
		signal
	);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await GetUserByEmailResponseDataSchema.parseAsync(response.data)
	};
}

/**
 * Send a request to the server to update a user.
 *
 * @param data The request data.
 * @returns A promise to the update user response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function updateUser(
	data: UpdateUserRequestData
): Promise<HttpResponse<UpdateUserResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendPatchRequest('/users', data, undefined);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await UpdateUserResponseDataSchema.parseAsync(response.data)
	};
}

/**
 * Send a request to the server to delete a user.
 *
 * @param data The request data.
 * @returns A promise to the delete user response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function deleteUser(
	data: DeleteUserRequestData
): Promise<HttpResponse<DeleteUserResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendDeleteRequest('/users', data, undefined);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await DeleteUserResponseDataSchema.parseAsync(response.data)
	};
}
