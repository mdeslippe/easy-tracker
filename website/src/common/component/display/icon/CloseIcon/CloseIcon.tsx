// React.
import { JSX } from 'react';

// Custom.
import { IconSize } from '@website/common/component/display';

/**
 * Properties for the {@link CloseIcon} component.
 */
export interface CloseIconProps {
	/**
	 * The id of the icon.
	 */
	id?: string;

	/**
	 * The class of the icon.
	 */
	className?: string;

	/**
	 * The color of the icon.
	 */
	color?: string;

	/**
	 * The background color of the icon.
	 */
	backgroundColor?: string;

	/**
	 * The size of the icon.
	 */
	size?: IconSize;
}

/**
 * A close icon component.
 *
 * @param props The component's properties.
 * @returns The close icon.
 */
export function CloseIcon({
	id,
	className,
	color,
	backgroundColor,
	size
}: CloseIconProps): JSX.Element {
	return (
		<svg
			id={id}
			className={['icon', `${size || IconSize.MEDIUM}-icon`, className].filter(Boolean).join(' ')}
			version='1.1'
			xmlns='http://www.w3.org/2000/svg'
			viewBox='0 0 24 24'
			stroke={color ?? 'var(--primary-color)'}
			strokeWidth='2'
			strokeLinecap='round'
		>
			<circle
				fill={backgroundColor ?? 'var(--secondary-color)'}
				cx='12'
				cy='12'
				r='11'
			/>
			<path d='M16 8L8 16M8 8L16 16' />
		</svg>
	);
}
