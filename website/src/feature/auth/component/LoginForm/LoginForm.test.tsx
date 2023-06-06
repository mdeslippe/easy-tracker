// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { LoginForm } from '@website/feature/auth/component/LoginForm/LoginForm';

// Test cases for rendering the login form component.
describe('Login Form component rendering', () => {
	it('Renders the login form without crashing', () => {
		// Render the component.
		render(<LoginForm />);
	});

	it('Renders all of the input fields inside of the login form', () => {
		// Render the component.
		const result = render(<LoginForm />);

		// Make sure the input fields were rendered.
		expect(result.container.querySelector('#username')).toBeDefined();
		expect(result.container.querySelector('#password')).toBeDefined();
	});

	it('Renders buttons to submit and reset the login form', () => {
		// Render the component.
		const result = render(<LoginForm />);

		// Make sure the buttons to submit and reset the form were rendered.
		expect(result.container.querySelector('input[type="submit"]')).toBeDefined();
		expect(result.container.querySelector('input[type="reset"]')).toBeDefined();
	});
});
