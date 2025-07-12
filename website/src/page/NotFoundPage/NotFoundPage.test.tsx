// Vitest.
import { describe, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router';

// React testing library.
import { render } from '@testing-library/react';

// React query.
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Pages.
import { NotFoundPage } from '@website/page/NotFoundPage/NotFoundPage';

// Test cases for rendering the not found page component.
describe('Not Found Page component rendering', () => {
	it('Renders the not found page component without crashing', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<NotFoundPage />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);
	});
});
