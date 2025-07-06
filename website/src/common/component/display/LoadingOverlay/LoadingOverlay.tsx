// React.
import { JSX } from 'react';

// Custom.
import { Spinner } from '@website/common/component/display';

// CSS.
import '@website/common/component/display/LoadingOverlay/loadingOverlay.css';

/**
 * A loading overlay component.
 *
 * @returns The loading overlay.
 */
export function LoadingOverlay(): JSX.Element {
	return (
		<div className='full-page-loading-overlay'>
			<Spinner />
		</div>
	);
}
