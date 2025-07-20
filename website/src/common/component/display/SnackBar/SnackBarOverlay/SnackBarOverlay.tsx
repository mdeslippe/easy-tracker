// React.
import { JSX, ComponentProps } from 'react';

// Context.
import { SnackType } from '@website/common/context';

// Hooks.
import { useSnackBar } from '@website/common/hook';

// Custom.
import { CheckIcon, InformationIcon, CloseIcon } from '@website/common/component/display';

// CSS.
import '@website/common/component/display/SnackBar/SnackBarOverlay/snackBarOverlay.css';

/**
 * Properties for the {@link SnackBarOverlay} component.
 */
export interface SnackBarOverlayProps extends Omit<ComponentProps<'div'>, 'id' | 'children'> {}

/**
 * A snackbar overlay component.
 *
 * @param props The component's properties.
 * @returns The snackbar overlay.
 */
export function SnackBarOverlay(props: SnackBarOverlayProps): JSX.Element {
	const snackbar = useSnackBar();

	return (
		<div
			id='snackbar'
			{...props}
		>
			{snackbar.snacks.map((snack) => (
				<div
					key={snack.id}
					className={`snack ${snack.type}-snack`}
					role='alert'
				>
					<button
						onClick={() => snackbar.removeSnack(snack.id)}
						type='button'
						title='Click to dismiss the alert'
					>
						{snack.type === SnackType.SUCCESS && <CheckIcon />}
						{snack.type === SnackType.NORMAL && <InformationIcon />}
						{snack.type === SnackType.ERROR && <CloseIcon />}
						<span>{snack.message}</span>
					</button>
				</div>
			))}
		</div>
	);
}
