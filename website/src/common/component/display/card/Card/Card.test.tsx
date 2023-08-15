// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { Card } from '@website/common/component/display/card/Card/Card';

// Test cases for rendering the card component.
describe('Card component rendering', () => {
	it('Renders an empty card without crashing', () => {
		// Render the component.
		render(<Card />);
	});

	it('Renders a single child inside of the card', () => {
		// Render the component.
		const result = render(<Card>Child Content</Card>);

		// Make sure the child was rendered.
		expect(result.getByText('Child Content')).toBeDefined();
	});

	it('Renders multiple children inside of the card', () => {
		// Render the component.
		const result = render(
			<Card>
				<div>Text box 1</div>
				<div>Text box 2</div>
				<div>Text box 3</div>
			</Card>
		);

		// Make sure the children were rendered.
		expect(result.getByText('Text box 1')).toBeDefined();
		expect(result.getByText('Text box 2')).toBeDefined();
		expect(result.getByText('Text box 3')).toBeDefined();
	});
});
