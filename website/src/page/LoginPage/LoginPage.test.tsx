// Vitest.
import { describe, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router';

// React testing library.
import { render } from '@testing-library/react';

// React query.
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Pages.
import { LoginPage } from '@website/page/LoginPage/LoginPage';

// Test cases for rendering the login page component.
describe('Login Page component rendering', () => {
	it('Renders the login page component without crashing', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<LoginPage />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);
	});
});
