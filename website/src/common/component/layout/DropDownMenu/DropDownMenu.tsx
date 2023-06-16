// React.
import { ComponentProps, ReactNode, useState } from 'react';

/**
 * Properties for the {@link DropDownMenu} component.
 */
export interface DropDownMenuProps extends ComponentProps<'div'> {
	/**
	 * A descriptive label that will be used for accessibility purposes.
	 */
	accessibilityLabel: string;

	/**
	 * The content that will be rendered inside of the drop down menu's button.
	 */
	buttonContent: ReactNode;

	/**
	 * The content that will be rendered when the drop down menu is expanded.
	 */
	children: ReactNode;
}

/**
 * A drop down menu component.
 *
 * @param props The component's properties.
 * @returns The drop down menu.
 */
export function DropDownMenu({
	accessibilityLabel,
	buttonContent,
	children,
	className,
	...props
}: DropDownMenuProps): JSX.Element {
	const [isOpen, setIsOpen] = useState<boolean>(false);

	return (
		<div
			{...props}
			className={className ? `${className} drop-down-menu` : 'drop-down-menu'}
		>
			<button
				className='drop-down-button'
				aria-haspopup='menu'
				aria-expanded={isOpen}
				aria-label={accessibilityLabel}
				onClick={() => setIsOpen((status) => !status)}
			>
				{buttonContent}
			</button>
			{isOpen && <div className='drop-down-content'>{children}</div>}
		</div>
	);
}
