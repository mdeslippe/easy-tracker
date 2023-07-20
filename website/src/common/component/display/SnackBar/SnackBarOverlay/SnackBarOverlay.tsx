// React.
import { ComponentProps } from 'react';

// Context.
import { SnackType } from '@website/common/context';

// Hooks.
import { useSnackBar } from '@website/common/hook';

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
						<img
							src={getSnackIconUrl(snack.type)}
							alt=''
						/>
						<span>{snack.message}</span>
					</button>
				</div>
			))}
		</div>
	);
}

/**
 * A function that can be used to get the icon that should be displayed based on the snack type.
 *
 * @param type The snack type.
 * @returns The icon that should be displayed.
 */
function getSnackIconUrl(type: SnackType) {
	switch (type) {
		case SnackType.SUCCESS:
			return '/images/icons/inverted-circle-check.svg';
		case SnackType.NORMAL:
			return '/images/icons/inverted-circle-info.svg';
		case SnackType.ERROR:
			return '/images/icons/inverted-circle-x.svg';
		default:
			return '/images/icons/inverted-circle-info.svg';
	}
}
