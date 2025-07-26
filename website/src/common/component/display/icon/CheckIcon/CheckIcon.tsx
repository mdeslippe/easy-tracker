// React.
import { JSX } from 'react';

// Custom.
import { IconSize } from '@website/common/component/display';

/**
 * Properties for the {@link CheckIcon} component.
 */
export interface CheckIconProps {
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
 * A check icon component.
 *
 * @param props The component's properties.
 * @returns The check icon.
 */
export function CheckIcon({
	id,
	className,
	color,
	backgroundColor,
	size
}: CheckIconProps): JSX.Element {
	return (
		<svg
			id={id}
			className={['icon', `${size ?? IconSize.MEDIUM}-icon`, className].filter(Boolean).join(' ')}
			version='1.1'
			xmlns='http://www.w3.org/2000/svg'
			viewBox='0 0 24 24'
			fill='none'
			stroke={color ?? 'var(--primary-color)'}
			strokeWidth='2'
			strokeLinecap='round'
			strokeLinejoin='round'
		>
			<circle
				fill={backgroundColor ?? 'var(--secondary-color)'}
				cx='12'
				cy='12'
				r='11'
			/>
			<path d='M6.5 12l4 4l7-7' />
		</svg>
	);
}
