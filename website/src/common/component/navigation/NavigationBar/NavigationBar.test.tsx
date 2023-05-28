// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { NavigationBar } from '@website/common/component/navigation/NavigationBar/NavigationBar';

// Test cases for rendering the navigation bar component.
describe('Navigation Bar component rendering', () => {
	it('Renders an empty navigation bar component without crashing', () => {
		// Render the component.
		render(<NavigationBar />);
	});

	it('Renders a single child inside of the navigation bar', () => {
		// Render the component.
		const result = render(
			<NavigationBar>
				<button>Page 1</button>
			</NavigationBar>
		);

		// Make sure the child was rendered.
		expect(result.getByText('Page 1')).toBeDefined();
	});

	it('Renders multiple children inside of the navigation bar', () => {
		// Render the component.
		const result = render(
			<NavigationBar>
				<button>Page 1</button>
				<button>Page 2</button>
				<button>Page 3</button>
			</NavigationBar>
		);

		// Make sure the children were rendered.
		expect(result.getByText('Page 1')).toBeDefined();
		expect(result.getByText('Page 2')).toBeDefined();
		expect(result.getByText('Page 3')).toBeDefined();
	});
});
