// Vitest.
import { describe, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import App from '@website/App';

// Test cases for rendering the main application component.
describe('Main application component rendering', () => {
	it('Renders the main application component without crashing', () => {
		render(<App />);
	});
});
