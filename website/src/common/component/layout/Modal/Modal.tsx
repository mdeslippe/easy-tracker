// React.
import { ComponentProps, useEffect, useRef } from 'react';

// Custom.
import { CloseIcon } from '@website/common/component/display';

// CSS.
import '@website/common/component/layout/Modal/modal.css';

/**
 * Properties for the {@link Modal} component.
 */
export interface ModalProps
	extends Omit<ComponentProps<'dialog'>, 'role' | 'aria-labelledby' | 'aria-describedby'> {
	/**
	 * The modal's id.
	 */
	id: string;

	/**
	 * The modal's title.
	 */
	title: string;

	/**
	 * If the modal is open.
	 */
	open: boolean;

	/**
	 * A function that will be invoked when the user wants to close the modal.
	 */
	onClose: () => void;
}

/**
 * A modal component.
 *
 * @param props The component's properties.
 * @returns The modal.
 */
export function Modal({
	className,
	children,
	id,
	title,
	open,
	onClose,
	onClick,
	...props
}: ModalProps): JSX.Element {
	const ref = useRef<HTMLDialogElement | null>(null);

	useEffect(() => {
		// If the ref is null.
		if (ref.current === null) {
			return;
		}

		// If the modal should be open, but it is closed.
		if (open && !ref.current.open) {
			ref.current.showModal();
		}

		// If the modal should be closed, but it is open.
		if (!open && ref.current.open) {
			ref.current.close();
		}
	}, [open, ref.current, ref.current?.open]);

	return (
		<dialog
			{...props}
			ref={ref}
			id={id}
			className={className ? `${className} modal` : 'modal'}
			role='dialog'
			aria-labelledby={`${id}-title`}
			aria-describedby={`${id}-description`}
			onClose={onClose}
			onClick={(event) => {
				// If the user clicked outside of the modal's content, the modal should be closed.
				if (event?.target instanceof HTMLDialogElement) {
					onClose();
					return;
				}

				// If a click event handler has been passed in, propagate the click event.
				if (onClick !== undefined) {
					onClick(event);
					return;
				}
			}}
		>
			<div className='modal-header'>
				<h1 id={`${id}-title`}>{title}</h1>
				<button
					title='Close'
					type='button'
					onClick={() => onClose()}
				>
					<CloseIcon color='var(--secondary-color)' />
				</button>
			</div>
			<div
				id={`${id}-description`}
				className='modal-body'
			>
				{children}
			</div>
		</dialog>
	);
}
