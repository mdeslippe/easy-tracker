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
						{/* If the snack is a success snack, render the success icon. */}
						{snack.type === SnackType.SUCCESS && (
							<CheckIcon
								color='var(--secondary-color)'
								backgroundColor='var(--success-snack-background)'
							/>
						)}

						{/* If the snack is a normal snack, render the normal icon. */}
						{snack.type === SnackType.NORMAL && (
							<InformationIcon
								color='var(--secondary-color)'
								backgroundColor='var(--normal-snack-background)'
							/>
						)}

						{/* If the snack is a error snack. render the error icon. */}
						{snack.type === SnackType.ERROR && (
							<CloseIcon
								color='var(--secondary-color)'
								backgroundColor='var(--error-snack-background)'
							/>
						)}

						{/* Render the snack message. */}
						<span>{snack.message}</span>
					</button>
				</div>
			))}
		</div>
	);
}
