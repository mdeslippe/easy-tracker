// Vitest.
import { describe, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React testing library.
import { render } from '@testing-library/react';

// React query.
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Pages.
import { SignUpPage } from '@website/page/SignUpPage/SignUpPage';

// Test cases for rendering the sign up page.
describe('Sign Up Page component rendering', () => {
	it('Renders the sign up page component without crashing', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<SignUpPage />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);
	});
});
