// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { SignUpForm } from '@website/feature/user/component/SignUpForm/SignUpForm';

// Test cases for rendering the sign up form component.
describe('Sign Up Form component rendering', () => {
	it('Renders the sign up form without crashing', () => {
		// Render the component.
		render(<SignUpForm />);
	});

	it('Renders all of the input fields inside of the sign up form', () => {
		// Render the component.
		const result = render(<SignUpForm />);

		// Make sure the input fields were rendered.
		expect(result.container.querySelector('#username')).toBeDefined();
		expect(result.container.querySelector('#email')).toBeDefined();
		expect(result.container.querySelector('#password')).toBeDefined();
		expect(result.container.querySelector('#confirmPassword')).toBeDefined();
	});

	it('Renders buttons to submit and reset the sign up form', () => {
		// Render the component.
		const result = render(<SignUpForm />);

		// Make sure the buttons to submit and reset the form were rendered.
		expect(result.container.querySelector('input[type="submit"]')).toBeDefined();
		expect(result.container.querySelector('input[type="reset"]')).toBeDefined();
	});
});
