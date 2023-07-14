// React.
import { Context, createContext } from 'react';

/**
 * An enumeration of all of the snack types.
 */
export enum SnackType {
	/**
	 * A snack type indicating that the snack will display a success message.
	 */
	SUCCESS,

	/**
	 * A snack type indicating that the snack will display a warning message.
	 */
	WARNING,

	/**
	 * A snack type indicating that the snack will display an error message.
	 */
	ERROR
}

/**
 * A snack that can be displayed in a snack bar.
 */
export type Snack = {
	/**
	 * A unique identifier that was generated for the snack.
	 */
	readonly id: string;

	/**
	 * The type of snack.
	 */
	readonly type: SnackType;

	/**
	 * The duration in milliseconds that the snack will be displayed for.
	 */
	readonly duration: number;

	/**
	 * The timeout handler for the snack.
	 */
	readonly timeout: ReturnType<typeof setTimeout>;

	/**
	 * The message that the snack will display.
	 */
	readonly message: string;
};

/**
 * The data that is stored in the {@link SnackBarContext}.
 */
export type SnackBarContextData = {
	/**
	 * The snacks that are currently in the snack bar.
	 */
	readonly snacks: Array<Snack>;

	/**
	 * A function that can be used to add a snack to the snack bar.
	 *
	 * @param snack The snack to add to the snack bar.
	 */
	readonly addSnack: (snack: Omit<Snack, 'id' | 'timeout'>) => void;

	/**
	 * A function that can be used to remove a snack from the snack bar.
	 *
	 * @param id The id of the snack to remove from the snack bar.
	 */
	readonly removeSnack: (id: string) => void;
};

/**
 * The snack bar context.
 */
export const SnackBarContext: Context<SnackBarContextData> = createContext<SnackBarContextData>({
	snacks: new Array<Snack>(),
	addSnack: () => {},
	removeSnack: () => {}
});
