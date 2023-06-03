// Axios.
import axios, { AxiosError, AxiosResponse } from 'axios';

/**
 * A simple http response type.
 */
export type HttpResponse<T> = {
	/**
	 * The http response data.
	 */
	data: T;

	/**
	 * The http response status.
	 */
	status: number;
};

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
): Promise<HttpResponse<unknown>> {
	// Create a response variable.
	let response: AxiosResponse<unknown, unknown>;

	try {
		// Send the request.
		response = await API.get(path, {
			params,
			signal,
			withCredentials: true
		});
	} catch (exception) {
		// If the error was an error response from the server.
		if (exception instanceof AxiosError && exception.response?.data !== undefined)
			response = exception.response;
		else throw exception;
	}

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
): Promise<HttpResponse<unknown>> {
	// Create a response variable.
	let response: AxiosResponse<unknown, unknown>;

	try {
		// Send the request.
		response = await API.post(path, body, {
			signal,
			headers: {
				'Content-Type': 'application/json'
			},
			withCredentials: true
		});
	} catch (exception) {
		// If the error was an error response from the server.
		if (exception instanceof AxiosError && exception.response?.data !== undefined)
			response = exception.response;
		else throw exception;
	}

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
): Promise<HttpResponse<unknown>> {
	// Create a response variable.
	let response: AxiosResponse<unknown, unknown>;

	try {
		// Send the request.
		response = await API.patch(path, body, {
			signal,
			headers: {
				'Content-Type': 'application/json'
			},
			withCredentials: true
		});
	} catch (exception) {
		// If the error was an error response from the server.
		if (exception instanceof AxiosError && exception.response?.data !== undefined)
			response = exception.response;
		else throw exception;
	}

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
): Promise<HttpResponse<unknown>> {
	// Create a response variable.
	let response: AxiosResponse<unknown, unknown>;

	try {
		// Send the request.
		response = await API.delete(path, {
			params,
			signal,
			withCredentials: true
		});
	} catch (exception) {
		// If the error was an error response from the server.
		if (exception instanceof AxiosError && exception.response?.data !== undefined)
			response = exception.response;
		else throw exception;
	}

	// Convert an empty string response to undefined.
	if (response.data === '') response.data = undefined;

	return response;
}
