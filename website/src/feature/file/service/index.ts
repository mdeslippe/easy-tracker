// Models.
import {
	CreateFileRequestData,
	CreateFileResponseData,
	CreateFileResponseDataSchema,
	DeleteFileRequestData,
	DeleteFileResponseData,
	DeleteFileResponseDataSchema,
	GetFileRequestData,
	GetFileResponseData,
	GetFileResponseDataSchema,
	UpdateFileRequestData,
	UpdateFileResponseData,
	UpdateFileResponseDataSchema
} from '@website/feature/file/model';

// Utils.
import {
	sendGetRequest,
	sendPostRequest,
	sendPatchRequest,
	sendDeleteRequest,
	HttpResponse
} from '@website/utility/http';

/**
 * Send a request to the server to create a new file.
 *
 * @param data The request data.
 * @returns A promise to the create file response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function createFile(
	data: CreateFileRequestData
): Promise<HttpResponse<CreateFileResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendPostRequest('/files', data, undefined);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await CreateFileResponseDataSchema.parseAsync(response.data)
	};
}

/**
 * Send a request to the server to get a file.
 *
 * @param data The request data.
 * @param signal A signal that can be used to abort the request.
 * @returns A promise to the get file response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function getFile(
	data: GetFileRequestData,
	signal: AbortSignal | undefined = undefined
): Promise<HttpResponse<GetFileResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendGetRequest(
		`/files/${data}`,
		undefined,
		signal
	);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await GetFileResponseDataSchema.parseAsync(response.data)
	};
}

/**
 * Send a request to the server to update a file.
 *
 * @param data The request data.
 * @returns A promise to the update file response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function updateFile(
	data: UpdateFileRequestData
): Promise<HttpResponse<UpdateFileResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendPatchRequest(
		`/files/${data.id}`,
		data,
		undefined
	);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await UpdateFileResponseDataSchema.parseAsync(response.data)
	};
}

/**
 * Send a request to the server to delete a file.
 *
 * @param data The request data.
 * @returns A promise to the delete file response.
 * @throws This function will throw an exception if:
 * - The server could not be reached.
 * - The servers sends an unsuccessful response.
 * - The response data could not be parsed.
 */
export async function deleteFile(
	data: DeleteFileRequestData
): Promise<HttpResponse<DeleteFileResponseData>> {
	// Send the request.
	const response: HttpResponse<unknown> = await sendDeleteRequest(
		`/files/${data}`,
		undefined,
		undefined
	);

	// Parse the response body and return the result.
	return {
		status: response.status,
		data: await DeleteFileResponseDataSchema.parseAsync(response.data)
	};
}
