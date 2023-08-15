// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { CardBody } from '@website/common/component/display/card/CardBody/CardBody';

// Test cases for rendering the card body component.
describe('Card Body component rendering', () => {
	it('Renders an empty card body without crashing', () => {
		// Render the component.
		render(<CardBody />);
	});

	it('Renders a single child inside of the card body', () => {
		// Render the component.
		const result = render(<CardBody>Child Content</CardBody>);

		// Make sure the child was rendered.
		expect(result.getByText('Child Content')).toBeDefined();
	});

	it('Renders multiple children inside of the card body', () => {
		// Render the component.
		const result = render(
			<CardBody>
				<div>Text box 1</div>
				<div>Text box 2</div>
				<div>Text box 3</div>
			</CardBody>
		);

		// Make sure the children were rendered.
		expect(result.getByText('Text box 1')).toBeDefined();
		expect(result.getByText('Text box 2')).toBeDefined();
		expect(result.getByText('Text box 3')).toBeDefined();
	});
});
