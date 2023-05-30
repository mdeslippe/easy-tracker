// Vitest.
import { describe, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React testing library.
import { render } from '@testing-library/react';

// Pages.
import { LandingPage } from '@website/page/LandingPage/LandingPage';

// Test cases for rendering the landing page component.
describe('Landing Page component rendering', () => {
	it('Renders the landing page component without crashing', () => {
		// Render the component.
		render(
			<BrowserRouter>
				<Routes>
					<Route
						path='/'
						element={<LandingPage />}
					/>
				</Routes>
			</BrowserRouter>
		);
	});
});
