// React.
import { useContext } from 'react';

// Context.
import { SnackBarContext, SnackBarContextData } from '@website/common/context';

/**
 * The data that will be returned from the useSnackBar hook.
 */
export type UseSnackBarResult = SnackBarContextData;

/**
 * A hook that can be used to manage the snack bar.
 *
 * @returns The result.
 */
export function useSnackBar(): UseSnackBarResult {
	return useContext(SnackBarContext);
}
