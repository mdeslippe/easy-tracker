/**
 * Capitalize the first letter of a string.
 *
 * @param value The value to modify.
 * @returns The modified value.
 */
export function capitalizeFirstLetter(value: string): string {
	return value.charAt(0).toUpperCase() + value.slice(1);
}
