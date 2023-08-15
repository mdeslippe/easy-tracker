// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { CardHeader } from '@website/common/component/display/card/CardHeader/CardHeader';

// Test cases for rendering the card header component.
describe('Card Header component rendering', () => {
	it('Renders an empty card header without crashing', () => {
		// Render the component.
		render(<CardHeader />);
	});

	it('Renders a single child inside of the card header', () => {
		// Render the component.
		const result = render(<CardHeader>Child Content</CardHeader>);

		// Make sure the child was rendered.
		expect(result.getByText('Child Content')).toBeDefined();
	});

	it('Renders multiple children inside of the card header', () => {
		// Render the component.
		const result = render(
			<CardHeader>
				<div>Text box 1</div>
				<div>Text box 2</div>
				<div>Text box 3</div>
			</CardHeader>
		);

		// Make sure the children were rendered.
		expect(result.getByText('Text box 1')).toBeDefined();
		expect(result.getByText('Text box 2')).toBeDefined();
		expect(result.getByText('Text box 3')).toBeDefined();
	});
});
