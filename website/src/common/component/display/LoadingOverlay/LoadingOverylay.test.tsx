// Vitest.
import { describe, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { LoadingOverlay } from '@website/common/component/display/LoadingOverlay/LoadingOverlay';

// Test cases for rendering the loading overlay component.
describe('Loading Overlay component rendering', () => {
	it('Renders a loading overlay component without crashing', () => {
		// Render the component.
		render(<LoadingOverlay />);
	});
});
