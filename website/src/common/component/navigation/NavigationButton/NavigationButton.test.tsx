// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { NavigationButton } from '@website/common/component/navigation/NavigationButton/NavigationButton';

// Test cases for rendering the navigation button component.
describe('Navigation Button component rendering', () => {
	it('Renders an empty navigation button component without crashing', () => {
		// Render the component.
		render(<NavigationButton href='/' />);
	});

	it('Renders a single child inside of the navigation button', () => {
		// Render the component.
		const result = render(<NavigationButton href='/'>Home</NavigationButton>);

		// Make sure the child was rendered.
		expect(result.getByText('Home')).toBeDefined();
	});

	it('Renders multiple children inside of the navigation button', () => {
		// Render the component.
		const result = render(
			<NavigationButton href='/'>
				<img
					src='image.png'
					alt='Home Icon'
				/>
				<span>Home</span>
			</NavigationButton>
		);

		// Make sure the children were rendered.
		expect(result.getByText('Home')).toBeDefined();
		expect(result.getByAltText('Home Icon')).toBeDefined();
	});
});
