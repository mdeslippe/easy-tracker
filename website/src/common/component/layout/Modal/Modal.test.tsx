// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { fireEvent, render, waitFor } from '@testing-library/react';

// Custom.
import { Modal } from '@website/common/component/layout/Modal/Modal';

// At the time of writing these tests, the testing library does not support dialogs. To get around
// this, we have to modify the dialog element's prototype.
HTMLDialogElement.prototype.show = function show(this: HTMLDialogElement) {
	this.open = true;
};
HTMLDialogElement.prototype.showModal = function showModal(this: HTMLDialogElement) {
	this.open = true;
};
HTMLDialogElement.prototype.close = function close(this: HTMLDialogElement) {
	this.open = false;
};

// Test cases for rendering the modal component.
describe('Modal component rendering', () => {
	it('Renders an empty closed modal without crashing', () => {
		// Render the component.
		render(
			<Modal
				id='test-modal'
				title=''
				onClose={() => {}}
				open={false}
			/>
		);
	});

	it('Renders an empty open modal without crashing', () => {
		// Render the component.
		render(
			<Modal
				id='test-modal'
				title=''
				onClose={() => {}}
				open={true}
			/>
		);
	});

	it('Renders a title inside of the modal', () => {
		// Render the component.
		const result = render(
			<Modal
				id='test-modal'
				title='This is my test modal!'
				onClose={() => {}}
				open={true}
			/>
		);

		// Make sure the title was rendered.
		expect(result.getByText('This is my test modal!')).toBeDefined();
	});

	it('Renders a single child inside of the modal', () => {
		// Render the component.
		const result = render(
			<Modal
				id='test-modal'
				title='This is my test modal!'
				onClose={() => {}}
				open={true}
			>
				<p>Test child</p>
			</Modal>
		);

		// Make sure the child was rendered.
		expect(result.getByText('Test child')).toBeDefined();
	});

	it('Renders multiple children inside of the modal', () => {
		// Render the component.
		const result = render(
			<Modal
				id='test-modal'
				title='This is my test modal!'
				onClose={() => {}}
				open={true}
			>
				<p>Test child 1</p>
				<p>Test child 2</p>
				<p>Test child 3</p>
			</Modal>
		);

		// Make sure the children were rendered.
		expect(result.getByText('Test child 1')).toBeDefined();
		expect(result.getByText('Test child 2')).toBeDefined();
		expect(result.getByText('Test child 3')).toBeDefined();
	});

	it('Closes the modal when the close button is clicked', async () => {
		// Declare a variable to store the state of the modal;
		let open = true;

		// Render the component.
		const result = render(
			<Modal
				id='test-modal'
				title='This is my test modal!'
				onClose={() => (open = false)}
				open={open}
			>
				<p>Test child</p>
			</Modal>
		);

		// Get the close button.
		const closeButton = result.container.querySelector('button');
		expect(closeButton).toBeDefined();

		// Click the close button.
		fireEvent.click(closeButton!);

		// Make sure the modal is closed.
		await waitFor(() => {
			expect(open).toBe(false);
		});
	});
});
