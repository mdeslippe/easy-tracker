// Vitest.
import { describe, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { Skeleton } from '@website/common/component/display/Skeleton/Skeleton';

// Test cases for rendering the skeleton component.
describe('Skeleton component rendering', () => {
	it('Renders the skeleton component without crashing', () => {
		// Render the component.
		render(<Skeleton />);
	});
});
