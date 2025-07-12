// Vitest.
import { describe, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router';

// React testing library.
import { render } from '@testing-library/react';

// Pages.
import { ErrorPage } from '@website/page/ErrorPage/ErrorPage';

// Test cases for rendering the error page component.
describe('Error Page component rendering', () => {
	it('Renders the error page component without crashing', () => {
		// Render the component.
		render(
			<BrowserRouter>
				<Routes>
					<Route
						path='/'
						element={<ErrorPage />}
					/>
				</Routes>
			</BrowserRouter>
		);
	});
});
