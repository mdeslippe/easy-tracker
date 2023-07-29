// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { ImageSelectionBox } from '@website/common/component/input/ImageSelectionBox/ImageSelectionBox';

// Test cases for rendering the image selection box component.
describe('Image Selection Box component rendering', () => {
	it('Renders an image selection box without crashing', () => {
		// Render the component.
		render(
			<ImageSelectionBox
				name='avatar'
				accept='image/png, image/jpeg'
				label='Select an Avatar'
				onSelect={() => {}}
			/>
		);
	});

	it('Renders an accessible label for the image selection box', () => {
		// Render the component.
		const result = render(
			<ImageSelectionBox
				name='avatar'
				accept='image/png, image/jpeg'
				label='Select an Avatar'
				onSelect={() => {}}
			/>
		);

		// Make sure the label is correct.
		expect(result.getByText('Select an Avatar').closest('label')?.htmlFor).toBe('avatar');

		// Make sure the input is correct.
		expect(result.container.querySelector('input')?.id).toBe('avatar');
		expect(result.container.querySelector('input')?.name).toBe('avatar');
	});
});
