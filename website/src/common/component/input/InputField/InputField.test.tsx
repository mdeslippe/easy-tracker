// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { InputField } from '@website/common/component/input/InputField/InputField';

// Test cases for rendering the input field component.
describe('Input Field component rendering', () => {
	it('Renders an input field without crashing', () => {
		// Render the component.
		render(
			<InputField
				label='Username'
				name='username'
				type='text'
			/>
		);
	});

	it('Renders an accessible label for the input field', () => {
		// Render the component.
		const result = render(
			<InputField
				label='exampleLabel'
				name='exampleName'
				type='text'
			/>
		);

		// Make sure the label is correct.
		expect(result.getByText('exampleLabel').closest('label')?.htmlFor).toBe('exampleName');

		// Make sure the input is correct.
		expect(result.container.querySelector('input')?.id).toBe('exampleName');
		expect(result.container.querySelector('input')?.name).toBe('exampleName');
	});

	it('Renders an error message when an error is passed in', () => {
		// Render the component.
		const result = render(
			<InputField
				label='exampleLabel'
				name='exampleName'
				type='text'
				error='A very bad error has occurred.'
			/>
		);

		// Make sure the error message was rendered.
		expect(result.getByText('A very bad error has occurred.')).toBeDefined();
	});
});
