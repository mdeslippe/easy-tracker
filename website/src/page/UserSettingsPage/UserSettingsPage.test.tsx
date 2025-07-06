// Vitest.
import { describe, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router';

// React testing library.
import { render } from '@testing-library/react';

// React query.
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Pages.
import { UserSettingsPage } from '@website/page/UserSettingsPage/UserSettingsPage';

// Test cases for rendering the user settings page component.
describe('User Settings Page component rendering', () => {
	it('Renders the user settings page component without crashing', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<UserSettingsPage />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);
	});
});
