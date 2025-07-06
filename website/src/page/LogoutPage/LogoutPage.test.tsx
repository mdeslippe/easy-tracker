// Vitest.
import { describe, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router';

// React testing library.
import { render } from '@testing-library/react';

// React query.
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Pages.
import { LogoutPage } from '@website/page/LogoutPage/LogoutPage';

// Test cases for rendering the logout page component.
describe('Logout Page component rendering', () => {
	it('Renders the logout page component without crashing', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<LogoutPage />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);
	});
});
