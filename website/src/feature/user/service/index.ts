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
	sendDeleteRequest
} from '@website/utility/http';

/**
 * Send a request to the server to create a new user.
 *
 * @param data The request data.
 * @returns A promise to the create user response.
 */
export async function createUser(data: CreateUserRequestData): Promise<CreateUserResponseData> {
	// Send the request.
	const response = await sendPostRequest('/users', data, undefined);

	// Parse the response body and return the result.
	return await CreateUserResponseDataSchema.parseAsync(response.data);
}

/**
 * Send a request to the server to get a user by their id.
 *
 * @param data The request data.
 * @param signal A signal that can be used to abort the request.
 * @returns A promise to the get user by id response.
 */
export async function getUserByID(
	data: GetUserByIDRequestData,
	signal: AbortSignal | undefined = undefined
): Promise<GetUserByIDResponseData> {
	// Send the request.
	const response = await sendGetRequest(`/users/id/${data}`, undefined, signal);

	// Parse the response body and return the result.
	return await GetUserByIDResponseDataSchema.parseAsync(response.data);
}

/**
 * Send a request to the server to get a user by their username.
 *
 * @param data The request data.
 * @param signal A signal that can be used to abort the request.
 * @returns A promise to the get user by username response.
 */
export async function getUserByUsername(
	data: GetUserByUsernameRequestData,
	signal: AbortSignal | undefined = undefined
): Promise<GetUserByUsernameResponseData> {
	// Send the request.
	const response = await sendGetRequest(`/users/username/${data}`, undefined, signal);

	// Parse the response body and return the result.
	return await GetUserByUsernameResponseDataSchema.parseAsync(response.data);
}

/**
 * Send a request to the server to get a user by their email address.
 *
 * @param data The request data.
 * @param signal A signal that can be used to abort the request.
 * @returns A promise to the get user by email response.
 */
export async function getUserByEmail(
	data: GetUserByEmailRequestData,
	signal: AbortSignal | undefined = undefined
): Promise<GetUserByEmailResponseData> {
	// Send the request.
	const response = await sendGetRequest(`/users/email/${data}`, undefined, signal);

	// Parse the response body and return the result.
	return await GetUserByEmailResponseDataSchema.parseAsync(response.data);
}

/**
 * Send a request to the server to update a user.
 *
 * @param data The request data.
 * @returns A promise to the update user response.
 */
export async function updateUser(data: UpdateUserRequestData): Promise<UpdateUserResponseData> {
	// Send the request.
	const response = await sendPatchRequest('/users', data, undefined);

	// Parse the response body and return the result.
	return await UpdateUserResponseDataSchema.parseAsync(response.data);
}

/**
 * Send a request to the server to delete a user.
 *
 * @param data The request data.
 * @returns A promise to the delete user response.
 */
export async function deleteUser(data: DeleteUserRequestData): Promise<DeleteUserResponseData> {
	// Send the request.
	const response = await sendDeleteRequest('/users', data, undefined);

	// Parse the response body and return the result.
	return await DeleteUserResponseDataSchema.parseAsync(response.data);
}
