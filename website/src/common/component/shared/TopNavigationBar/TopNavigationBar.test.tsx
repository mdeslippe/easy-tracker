// Vitest.
import { describe, expect, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React testing library.
import { render } from '@testing-library/react';

// React query.
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Custom.
import { TopNavigationBar } from '@website/common/component/shared/TopNavigationBar/TopNavigationBar';

// Test cases for rendering the top navigation bar.
describe('Top Navigation Bar component rendering', () => {
	it('Renders the top navigation bar component without crashing', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<TopNavigationBar />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);
	});

	it('Renders navigation buttons inside the top navigation bar', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		const result = render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<TopNavigationBar />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);

		// Make sure the navigation buttons were rendered correctly.
		expect(result.getByText('Easy Tracker').closest('a')?.href).toMatch(/\/$/);
		expect(result.getByText('Login').closest('a')?.href).toMatch(/\/login$/);
		expect(result.getByText('Sign Up').closest('a')?.href).toMatch(/\/signup$/);
	});
});
