// Vitest.
import { describe, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { SignUpPage } from '@website/page/SignUpPage/SignUpPage';

// Test cases for rendering the sign up page.
describe('Sign Up Page component rendering', () => {
	it('Renders the sign up page component without crashing', () => {
		// Render the component.
		render(<SignUpPage />);
	});
});
