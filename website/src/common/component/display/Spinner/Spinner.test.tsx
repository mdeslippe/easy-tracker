// Vitest.
import { describe, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { Spinner } from '@website/common/component/display/Spinner/Spinner';

// Test cases for rendering the spinner component.
describe('Spinner component rendering', () => {
	it('Renders a spinner component without crashing', () => {
		// Render the component.
		render(<Spinner />);
	});
});
