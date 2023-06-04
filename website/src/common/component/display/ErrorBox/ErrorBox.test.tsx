// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { ErrorBox } from '@website/common/component/display/ErrorBox/ErrorBox';

// Test cases for rendering the error box component.
describe('Error Box component rendering', () => {
	it('Renders an error box component without crashing', () => {
		// Render the component.
		render(
			<ErrorBox
				message='A very bad error has occurred.'
				onClose={() => {}}
			/>
		);
	});

	it('Renders the error message inside of an error box', () => {
		// Render the component.
		const result = render(
			<ErrorBox
				message='A very bad error has occurred.'
				onClose={() => {}}
			/>
		);

		// Make sure the message was rendered.
		expect(result.getByText('A very bad error has occurred.')).toBeDefined();
	});

	it('Renders a button to close the error box', () => {
		// Render the component.
		const result = render(
			<ErrorBox
				message='A very bad error has occurred.'
				onClose={() => {}}
			/>
		);

		// Make sure the button was rendered.
		expect(result.container.querySelector('button')).toBeDefined();
	});

	it('Invokes the onClose property when the close button is clicked on', () => {
		let wasClicked: boolean = false;

		// Render the component.
		const result = render(
			<ErrorBox
				message='A very bad error has occurred.'
				onClose={() => (wasClicked = true)}
			/>
		);

		// Make sure the button has not been clicked yet.
		expect(wasClicked).toBeFalsy();

		// Click the button.
		result.container.querySelector('button')?.click();

		// Make sure the button was clicked.
		expect(wasClicked).toBeTruthy();
	});
});
