// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { IconSize } from '@website/common/component/display/icon';
import { InformationIcon } from '@website/common/component/display/icon/InformationIcon/InformationIcon';

// Test cases for rendering the information icon component.
describe('Information Icon component rendering', () => {
	it('Renders the information icon component without crashing', () => {
		// Render the component.
		render(<InformationIcon />);
	});

	it('Renders the small information icon component when the size is set to small', () => {
		// Render the component.
		const result = render(<InformationIcon size={IconSize.SMALL} />);

		// Make sure the small information icon component was rendered.
		expect(result.container.querySelector(`.${IconSize.SMALL}-icon`)).toBeDefined();
		expect(result.container.querySelector(`.${IconSize.MEDIUM}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.LARGE}-icon`)).toBeNull();
	});

	it('Renders the medium information icon component when the size is set to medium', () => {
		// Render the component.
		const result = render(<InformationIcon size={IconSize.MEDIUM} />);

		// Make sure the medium information icon component was rendered.
		expect(result.container.querySelector(`.${IconSize.SMALL}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.MEDIUM}-icon`)).toBeDefined();
		expect(result.container.querySelector(`.${IconSize.LARGE}-icon`)).toBeNull();
	});

	it('Renders the large information icon component when the size is set to large', () => {
		// Render the component.
		const result = render(<InformationIcon size={IconSize.LARGE} />);

		// Make sure the large information icon component was rendered.
		expect(result.container.querySelector(`.${IconSize.SMALL}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.MEDIUM}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.LARGE}-icon`)).toBeDefined();
	});
});
