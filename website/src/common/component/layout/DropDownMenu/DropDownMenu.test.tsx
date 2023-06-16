// Vitest.
import { describe, expect, it } from 'vitest';

// React.
import { Fragment } from 'react';

// React testing library.
import { act, render } from '@testing-library/react';

// Custom.
import { DropDownMenu } from '@website/common/component/layout/DropDownMenu/DropDownMenu';

// Test cases for rendering the drop down menu component.
describe('Drop Down Menu component rendering', () => {
	it('Renders an empty drop down menu without crashing', () => {
		// Render the component.
		render(
			<DropDownMenu
				accessibilityLabel='Test Menu'
				buttonContent={<Fragment />}
			>
				<Fragment />
			</DropDownMenu>
		);
	});

	it('Does not render the children by default', () => {
		// Render the component.
		const result = render(
			<DropDownMenu
				accessibilityLabel='Test Menu'
				buttonContent={<Fragment />}
			>
				<p>This should not be visible.</p>
			</DropDownMenu>
		);

		// Make sure the children were not rendered.
		expect(result.container.querySelector('p')?.textContent).toBeUndefined();
	});

	it('Renders the children after the drop down menu has been clicked on', () => {
		// Render the component.
		const result = render(
			<DropDownMenu
				accessibilityLabel='Test Menu'
				buttonContent={<Fragment />}
			>
				<p>This should be visible after clicking the button.</p>
			</DropDownMenu>
		);

		// Make sure the children were not rendered.
		expect(result.container.querySelector('p')?.textContent).toBeUndefined();

		// Find the button and click it.
		act(() => {
			expect(result.container.querySelector('button')).toBeDefined();
			result.container.querySelector('button')?.click();
		});

		// Make sure the children were rendered.
		expect(result.container.querySelector('p')?.textContent).toEqual(
			'This should be visible after clicking the button.'
		);
	});

	it('Hides the children when the drop down menu is open, and has been click on again', () => {
		// Render the component.
		const result = render(
			<DropDownMenu
				accessibilityLabel='Test Menu'
				buttonContent={<Fragment />}
			>
				<p>This should be visible after clicking the button.</p>
			</DropDownMenu>
		);

		// Make sure the children were not rendered.
		expect(result.container.querySelector('p')?.textContent).toBeUndefined();

		// Find the button and click it.
		act(() => {
			expect(result.container.querySelector('button')).toBeDefined();
			result.container.querySelector('button')?.click();
		});

		// Make sure the children were rendered.
		expect(result.container.querySelector('p')?.textContent).toEqual(
			'This should be visible after clicking the button.'
		);

		// Find the button and click it.
		act(() => {
			expect(result.container.querySelector('button')).toBeDefined();
			result.container.querySelector('button')?.click();
		});

		// Make sure the children were not rendered.
		expect(result.container.querySelector('p')?.textContent).toBeUndefined();
	});
});
