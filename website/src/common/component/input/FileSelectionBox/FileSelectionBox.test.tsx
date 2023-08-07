// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { FileSelectionBox } from '@website/common/component/input/FileSelectionBox/FileSelectionBox';

// Test cases for rendering the file selection box component.
describe('File Selection Box component rendering', () => {
	it('Renders a file selection box without crashing', () => {
		// Render the component.
		render(
			<FileSelectionBox
				name='avatar'
				accept='file/png, file/jpeg'
				label='Select an Avatar'
				onSelect={() => {}}
			/>
		);
	});

	it('Renders an accessible label for the file selection box', () => {
		// Render the component.
		const result = render(
			<FileSelectionBox
				name='avatar'
				accept='file/png, file/jpeg'
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
