// Axios.
import axios, { AxiosResponse } from 'axios';

/**
 * An axios instance that can be used to send requests to the API.
 */
const API = axios.create({
	baseURL: import.meta.env.VITE_API_SERVER_URL
});

/**
 * Send a GET request to the API.
 *
 * @param path The request's path.
 * @param params The request's query parameters.
 * @param signal An abort signal that can be used to abort the request.
 * @returns A promise to the request's response.
 */
export async function sendGetRequest(
	path: string,
	params: object | undefined = undefined,
	signal: AbortSignal | undefined = undefined
): Promise<AxiosResponse<unknown, unknown>> {
	// Send the request.
	const response = await API.get(path, {
		params,
		signal,
		withCredentials: true
	});

	// Convert an empty string response to undefined.
	if (response.data === '') response.data = undefined;

	return response;
}

/**
 * Send a POST request to the API.
 *
 * @param path The request's path.
 * @param body The request's body.
 * @param signal An abort signal that can be used to abort the request.
 * @returns A promise to the request's response.
 */
export async function sendPostRequest(
	path: string,
	body: object | undefined = undefined,
	signal: AbortSignal | undefined = undefined
): Promise<AxiosResponse<unknown, unknown>> {
	// Send the request.
	const response = await API.post(path, body, {
		signal,
		headers: {
			'Content-Type': 'application/json'
		},
		withCredentials: true
	});

	// Convert an empty string response to undefined.
	if (response.data === '') response.data = undefined;

	return response;
}

/**
 * Send a PATCH request to the API.
 *
 * @param path The request's path.
 * @param body The request's body.
 * @param signal An abort signal that can be used to abort the request.
 * @returns A promise to the request's response.
 */
export async function sendPatchRequest(
	path: string,
	body: object | undefined = undefined,
	signal: AbortSignal | undefined = undefined
): Promise<AxiosResponse<unknown, unknown>> {
	// Send the request.
	const response = await API.patch(path, body, {
		signal,
		headers: {
			'Content-Type': 'application/json'
		},
		withCredentials: true
	});

	// Convert an empty string response to undefined.
	if (response.data === '') response.data = undefined;

	return response;
}

/**
 * Send a DELETE request to the API.
 *
 * @param path The request's path.
 * @param params The request's query parameters.
 * @param signal An abort signal that can be used to abort the request.
 * @returns A promise to the request's response.
 */
export async function sendDeleteRequest(
	path: string,
	params: object | undefined = undefined,
	signal: AbortSignal | undefined = undefined
): Promise<AxiosResponse<unknown, unknown>> {
	// Send the request.
	const response = await API.delete(path, {
		params,
		signal,
		withCredentials: true
	});

	// Convert an empty string response to undefined.
	if (response.data === '') response.data = undefined;

	return response;
}
