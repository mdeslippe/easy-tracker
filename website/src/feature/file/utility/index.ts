// Utils.
import { SERVER_BASE_URL } from '@website/utility/http';

/**
 * A function to get the url for a file by id.
 *
 * @param id The id of the file.
 * @returns The url for the file.
 */
export function getFileUrl(id: number): string {
	return `${SERVER_BASE_URL}/files/${id}?raw=true`;
}
