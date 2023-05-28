// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { MaxWidthContainer } from '@website/common/component/layout/MaxWidthContainer/MaxWidthContainer';

// Test cases for rendering the max width container component.
describe('Max Width Container component rendering', () => {
	it('Renders an empty max width container without crashing', () => {
		// Render the component.
		render(<MaxWidthContainer />);
	});

	it('Renders a single child inside of the max width container', () => {
		// Render the component.
		const result = render(<MaxWidthContainer>Contained text</MaxWidthContainer>);

		// Make sure the child was rendered.
		expect(result.getByText('Contained text')).toBeDefined();
	});

	it('Renders multiple children inside of the max width container', () => {
		// Render the component.
		const result = render(
			<MaxWidthContainer>
				<div>Text box 1</div>
				<div>Text box 2</div>
				<div>Text box 3</div>
			</MaxWidthContainer>
		);

		// Make sure the children were rendered.
		expect(result.getByText('Text box 1')).toBeDefined();
		expect(result.getByText('Text box 2')).toBeDefined();
		expect(result.getByText('Text box 3')).toBeDefined();
	});
});
