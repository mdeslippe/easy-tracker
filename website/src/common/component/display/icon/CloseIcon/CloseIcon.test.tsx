// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { IconSize } from '@website/common/component/display/icon';
import { CloseIcon } from '@website/common/component/display/icon/CloseIcon/CloseIcon';

// Test cases for rendering the close icon component.
describe('Close Icon component rendering', () => {
	it('Renders the close icon component without crashing', () => {
		// Render the component.
		render(<CloseIcon />);
	});

	it('Renders the small close icon component when the size is set to small', () => {
		// Render the component.
		const result = render(<CloseIcon size={IconSize.SMALL} />);

		// Make sure the small close icon component was rendered.
		expect(result.container.querySelector(`.${IconSize.SMALL}-icon`)).toBeDefined();
		expect(result.container.querySelector(`.${IconSize.MEDIUM}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.LARGE}-icon`)).toBeNull();
	});

	it('Renders the medium close icon component when the size is set to medium', () => {
		// Render the component.
		const result = render(<CloseIcon size={IconSize.MEDIUM} />);

		// Make sure the medium close icon component was rendered.
		expect(result.container.querySelector(`.${IconSize.SMALL}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.MEDIUM}-icon`)).toBeDefined();
		expect(result.container.querySelector(`.${IconSize.LARGE}-icon`)).toBeNull();
	});

	it('Renders the large close icon component when the size is set to large', () => {
		// Render the component.
		const result = render(<CloseIcon size={IconSize.LARGE} />);

		// Make sure the large close icon component was rendered.
		expect(result.container.querySelector(`.${IconSize.SMALL}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.MEDIUM}-icon`)).toBeNull();
		expect(result.container.querySelector(`.${IconSize.LARGE}-icon`)).toBeDefined();
	});
});
