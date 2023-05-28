// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { NavigationGroup } from '@website/common/component/navigation/NavigationGroup/NavigationGroup';

// Test cases for rendering the navigation group component.
describe('Navigation Group component rendering', () => {
	it('Renders an empty navigation group component without crashing', () => {
		// Render the component.
		render(<NavigationGroup />);
	});

	it('Renders a single child inside of the navigation group', () => {
		// Render the component.
		const result = render(
			<NavigationGroup>
				<a href='/'>Page 1</a>
			</NavigationGroup>
		);

		// Make sure the child was rendered.
		expect(result.getByText('Page 1')).toBeDefined();
	});

	it('Renders multiple children inside of the navigation group', () => {
		// Render the component.
		const result = render(
			<NavigationGroup>
				<a href='/'>Page 1</a>
				<a href='/'>Page 2</a>
				<a href='/'>Page 3</a>
			</NavigationGroup>
		);

		// Make sure the children were rendered.
		expect(result.getByText('Page 1')).toBeDefined();
		expect(result.getByText('Page 2')).toBeDefined();
		expect(result.getByText('Page 3')).toBeDefined();
	});
});
