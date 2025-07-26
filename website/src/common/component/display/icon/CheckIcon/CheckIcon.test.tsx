// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { IconSize } from '@website/common/component/display/icon';
import { CheckIcon } from '@website/common/component/display/icon/CheckIcon/CheckIcon';

// Test cases for rendering the check icon component.
describe('Check Icon component rendering', () => {
	it('Renders the check icon component without crashing', () => {
		// Render the component.
		render(<CheckIcon />);
	});

	it('Renders the small check icon component when the size is set to small', () => {
		// Render the component.
		const result = render(<CheckIcon size={IconSize.SMALL} />);

		// Make sure the small check icon component was rendered.
		expect(result.container.querySelector(`.${IconSize.SMALL}-icon`)).toBeDefined();
		expect(result.container.querySelector(`.${IconSize.MEDIUM}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.LARGE}-icon`)).toBeNull();
	});

	it('Renders the medium check icon component when the size is set to medium', () => {
		// Render the component.
		const result = render(<CheckIcon size={IconSize.MEDIUM} />);

		// Make sure the medium check icon component was rendered.
		expect(result.container.querySelector(`.${IconSize.SMALL}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.MEDIUM}-icon`)).toBeDefined();
		expect(result.container.querySelector(`.${IconSize.LARGE}-icon`)).toBeNull();
	});

	it('Renders the large check icon component when the size is set to large', () => {
		// Render the component.
		const result = render(<CheckIcon size={IconSize.LARGE} />);

		// Make sure the large check icon component was rendered.
		expect(result.container.querySelector(`.${IconSize.SMALL}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.MEDIUM}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.LARGE}-icon`)).toBeDefined();
	});
});
