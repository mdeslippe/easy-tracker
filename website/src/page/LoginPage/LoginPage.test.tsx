// Vitest.
import { describe, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Pages.
import { LoginPage } from '@website/page/LoginPage/LoginPage';

// Test cases for rendering the login page component.
describe('Login Page component rendering', () => {
	it('Renders the login page component without crashing', () => {
		// Render the component.
		render(<LoginPage />);
	});
});
