// Vitest.
import { describe, expect, it } from 'vitest';

// React.
import { Fragment } from 'react';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React testing library.
import { render } from '@testing-library/react';

// React query.
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Hooks.
import { getAuthenticationStatusQueryKey } from '@website/feature/auth/hook';

// Custom.
import { AuthenticatedRoute } from '@website/common/component/navigation/AuthenticatedRoute/AuthenticatedRoute';

// Test cases for rendering the authenticated route component.
describe('Authenticated Route component rendering', () => {
	it('Renders the authenticated route component without crashing', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={
								<AuthenticatedRoute
									redirectTo='/'
									Component={Fragment}
								/>
							}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);
	});

	it('Renders the component when the user is authenticated', () => {
		// Create a react query client.
		const queryClient = new QueryClient();
		queryClient.setQueryData(getAuthenticationStatusQueryKey(), { status: 200, data: true });

		// Render the component.
		const result = render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={
								<AuthenticatedRoute
									redirectTo='/'
									Component={() => <p>Authenticated users can see this!</p>}
								/>
							}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);

		// Make sure the component was rendered.
		expect(result.container.querySelector('p')?.textContent).toBeDefined();
	});

	it('Does not render the component when the user is not authenticated', () => {
		// Create a react query client.
		const queryClient = new QueryClient();
		queryClient.setQueryData(getAuthenticationStatusQueryKey(), { status: 200, data: false });

		// Render the component.
		const result = render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={
								<AuthenticatedRoute
									redirectTo='/'
									Component={() => <p>Authenticated users can see this!</p>}
								/>
							}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);

		// Make sure the component was not rendered.
		expect(result.container.querySelector('p')?.textContent).toBeUndefined();
	});
});
