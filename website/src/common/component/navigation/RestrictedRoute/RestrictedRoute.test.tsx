// Vitest.
import { describe, expect, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React testing library.
import { render } from '@testing-library/react';

// Custom.
import { RestrictedRoute } from '@website/common/component/navigation/RestrictedRoute/RestrictedRoute';

// Test cases for rendering the restricted route component.
describe('Restricted Route component rendering', () => {
	it('Renders the restricted route component where the route is permitted without crashing', () => {
		// Render the component.
		render(
			<BrowserRouter>
				<Routes>
					<Route
						path='/'
						element={
							<RestrictedRoute
								permitted={true}
								redirectTo='/'
								children={null}
							/>
						}
					/>
				</Routes>
			</BrowserRouter>
		);
	});

	it('Renders the restricted route component where the route is not permitted without crashing', () => {
		// Render the component.
		render(
			<BrowserRouter>
				<Routes>
					<Route
						path='/'
						element={
							<RestrictedRoute
								permitted={false}
								redirectTo='/'
								children={null}
							/>
						}
					/>
				</Routes>
			</BrowserRouter>
		);
	});

	it('Renders the children when the route is permitted', () => {
		// Render the component.
		const result = render(
			<BrowserRouter>
				<Routes>
					<Route
						path='/'
						element={
							<RestrictedRoute
								permitted={true}
								redirectTo='/'
							>
								<p>Test Content</p>
							</RestrictedRoute>
						}
					/>
				</Routes>
			</BrowserRouter>
		);

		// Make sure the content was rendered.
		expect(result.container.querySelector('p')?.textContent).toBe('Test Content');
	});

	it('Does not render the children when the router is not permitted', () => {
		// Render the component.
		const result = render(
			<BrowserRouter>
				<Routes>
					<Route
						path='/'
						element={
							<RestrictedRoute
								permitted={false}
								redirectTo='/'
							>
								<p>Test Content</p>
							</RestrictedRoute>
						}
					/>
				</Routes>
			</BrowserRouter>
		);

		// Make sure the content was not rendered.
		expect(result.container.querySelector('p')?.textContent).toBeUndefined();
	});
});
