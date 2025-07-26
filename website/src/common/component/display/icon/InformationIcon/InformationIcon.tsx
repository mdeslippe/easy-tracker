// React.
import { JSX } from 'react';

// Custom.
import { IconSize } from '@website/common/component/display';

/**
 * Properties for the {@link InformationIcon} component.
 */
export interface InformationIconProps {
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
 * An information icon component.
 *
 * @param props The component's properties.
 * @returns The information icon.
 */
export function InformationIcon({
	id,
	className,
	color,
	backgroundColor,
	size
}: InformationIconProps): JSX.Element {
	return (
		<svg
			id={id}
			className={['icon', `${size || IconSize.MEDIUM}-icon`, className].filter(Boolean).join(' ')}
			version='1.1'
			xmlns='http://www.w3.org/2000/svg'
			viewBox='0 0 24 24'
			stroke={color || 'var(--primary-color)'}
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
			<circle
				cx='12'
				cy='7'
				r='0.25'
			/>
			<line
				x1='12'
				y1='11'
				x2='12'
				y2='18'
			/>
		</svg>
	);
}
