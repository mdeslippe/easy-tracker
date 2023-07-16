// React.
import { ReactNode, useState } from 'react';

// Hooks.
import { useCounter } from '@website/common/hook';

// Context.
import { Snack, SnackBarContext, SnackBarContextData } from '@website/common/context';

/**
 * Properties for the {@link SnackBarProvider} component.
 */
export interface SnackBarProviderProps {
	/**
	 * The nodes that will be rendered as the component's children.
	 */
	children: ReactNode;
}

/**
 * A component that can be used to provide snack bar services to all child components.
 *
 * @param props The component's properties.
 * @returns The snack bar provider.
 */
export function SnackBarProvider({ children }: SnackBarProviderProps): JSX.Element {
	const [snacks, setSnacks] = useState<Array<Snack>>(new Array<Snack>());
	const getNextID = useCounter();

	const providerValue: SnackBarContextData = {
		snacks: snacks,
		addSnack: (snack) => {
			const id = String(getNextID());
			const timeout = setTimeout(() => providerValue.removeSnack(id), snack.duration);
			setSnacks((currentSnacks) => [...currentSnacks, { ...snack, id, timeout }]);
		},
		removeSnack: (id) => {
			setSnacks((currentSnacks) =>
				currentSnacks.filter((snack) => {
					if (snack.id === id) clearTimeout(snack.timeout);
					return snack.id !== id;
				})
			);
		}
	};

	return <SnackBarContext.Provider value={providerValue}>{children}</SnackBarContext.Provider>;
}
