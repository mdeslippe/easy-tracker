// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { SnackBarProvider } from '@website/common/component/display/SnackBar/SnackBarProvider/SnackBarProvider';

// Test cases for rendering the snack bar provider component.
describe('Snack Bar Provider component rendering', () => {
	it('Renders the snack bar provider component without crashing', () => {
		// Render the component.
		render(<SnackBarProvider />);
	});

	it('Renders a single child in the snack bar provider component', () => {
		// Render the component.
		const result = render(
			<SnackBarProvider>
				<p>Test content</p>
			</SnackBarProvider>
		);

		// Make sure the content was rendered.
		expect(result.getByText('Test content')).toBeDefined();
	});

	it('Renders multiple children in the snack bar provider component', () => {
		// Render the component.
		const result = render(
			<SnackBarProvider>
				<p>Test content 1</p>
				<p>Test content 2</p>
				<p>Test content 3</p>
			</SnackBarProvider>
		);

		// Make sure the content was rendered.
		expect(result.getByText('Test content 1')).toBeDefined();
		expect(result.getByText('Test content 2')).toBeDefined();
		expect(result.getByText('Test content 3')).toBeDefined();
	});
});
