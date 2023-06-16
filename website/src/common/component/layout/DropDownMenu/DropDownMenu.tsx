// React.
import { ComponentProps, ReactNode, useState } from 'react';

// CSS.
import '@website/common/component/layout/DropDownMenu/dropDownMenu.css';

/**
 * Properties for the {@link DropDownMenu} component.
 */
export interface DropDownMenuProps extends ComponentProps<'div'> {
	/**
	 * A descriptive label that will be used for accessibility purposes.
	 */
	accessibilityLabel: string;

	/**
	 * The alignment of the expanded content, relative to the bottom of the button.
	 */
	align: 'left' | 'center' | 'right';

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
	align,
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
			{isOpen && <div className={`drop-down-content align-${align}`}>{children}</div>}
		</div>
	);
}
