// Vitest.
import { describe, expect, it } from 'vitest';

// React testing library.
import { render } from '@testing-library/react';

// Context.
import { Snack, SnackBarContext, SnackType } from '@website/common/context';

// Custom.
import { SnackBarOverlay } from '@website/common/component/display/SnackBar/SnackBarOverlay/SnackBarOverlay';

// Test cases for rendering the snack bar overlay component.
describe('Snack Bar Overlay component rendering', () => {
	it('Renders the snack bar overlay component outside of a context provider without crashing', () => {
		// Render the component.
		render(<SnackBarOverlay />);
	});

	it('Renders the snack bar overlay component inside of a context provider without crashing', () => {
		// Render the component.
		render(
			<SnackBarContext.Provider
				value={{ snacks: new Array<Snack>(), addSnack: () => {}, removeSnack: () => {} }}
			>
				<SnackBarOverlay />
			</SnackBarContext.Provider>
		);
	});

	it('Does not render any snacks in the snack bar overlay when the snackbar is empty', () => {
		// Render the component.
		const result = render(
			<SnackBarContext.Provider
				value={{ snacks: new Array<Snack>(), addSnack: () => {}, removeSnack: () => {} }}
			>
				<SnackBarOverlay />
			</SnackBarContext.Provider>
		);

		// Make sure no snacks were rendered.
		expect(result.container.children[0].children.length).toBe(0);
	});

	it('Renders a single snack in the snack bar overlay', () => {
		// Render the component.
		const result = render(
			<SnackBarContext.Provider
				value={{
					snacks: [
						{
							id: '1',
							timeout: undefined!,
							duration: Infinity,
							message: 'Test snack',
							type: SnackType.NORMAL
						}
					],
					addSnack: () => {},
					removeSnack: () => {}
				}}
			>
				<SnackBarOverlay />
			</SnackBarContext.Provider>
		);

		// Make sure the snack was rendered.
		expect(result.container.children[0].children.length).toBe(1);
		expect(result.getByText('Test snack')).toBeDefined();
	});

	it('Renders multiple snacks in the snack bar overlay', () => {
		// Render the component.
		const result = render(
			<SnackBarContext.Provider
				value={{
					snacks: [
						{
							id: '1',
							timeout: undefined!,
							duration: Infinity,
							message: 'Test snack #1',
							type: SnackType.SUCCESS
						},
						{
							id: '2',
							timeout: undefined!,
							duration: Infinity,
							message: 'Test snack #2',
							type: SnackType.NORMAL
						},
						{
							id: '3',
							timeout: undefined!,
							duration: Infinity,
							message: 'Test snack #3',
							type: SnackType.ERROR
						}
					],
					addSnack: () => {},
					removeSnack: () => {}
				}}
			>
				<SnackBarOverlay />
			</SnackBarContext.Provider>
		);

		// Make sure the snacks were rendered.
		expect(result.container.children[0].children.length).toBe(3);
		expect(result.getByText('Test snack #1')).toBeDefined();
		expect(result.getByText('Test snack #2')).toBeDefined();
		expect(result.getByText('Test snack #3')).toBeDefined();
	});
});
