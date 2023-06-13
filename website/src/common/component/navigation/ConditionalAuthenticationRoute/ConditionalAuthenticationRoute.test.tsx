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
import { ConditionalAuthenticationRoute } from '@website/common/component/navigation/ConditionalAuthenticationRoute/ConditionalAuthenticationRoute';

describe('Conditional Authentication Route component rendering', () => {
	it('Renders the conditional authentication route component without crashing', () => {
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
								<ConditionalAuthenticationRoute
									AuthenticatedComponent={Fragment}
									UnauthenticatedComponent={Fragment}
								/>
							}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);
	});

	it('Renders the authenticated component when the user is authenticated', () => {
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
								<ConditionalAuthenticationRoute
									AuthenticatedComponent={() => <p>You are authenticated!</p>}
									UnauthenticatedComponent={() => (
										<p>You are not authenticated!</p>
									)}
								/>
							}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);

		expect(result.getByText('You are authenticated!')).toBeDefined();
	});

	it('Renders the unauthenticated component when the user is not authenticated', () => {
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
								<ConditionalAuthenticationRoute
									AuthenticatedComponent={() => <p>You are authenticated!</p>}
									UnauthenticatedComponent={() => (
										<p>You are not authenticated!</p>
									)}
								/>
							}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);

		expect(result.getByText('You are not authenticated!')).toBeDefined();
	});
});
