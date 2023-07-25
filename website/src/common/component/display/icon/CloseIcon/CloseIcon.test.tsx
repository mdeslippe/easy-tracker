// Vitest.
import { describe, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { CloseIcon } from '@website/common/component/display/icon/CloseIcon/CloseIcon';

// Test cases for rendering the close icon component.
describe('Close Icon component rendering', () => {
	it('Renders the close icon without crashing', () => {
		// Render the component.
		render(<CloseIcon />);
	});
});
