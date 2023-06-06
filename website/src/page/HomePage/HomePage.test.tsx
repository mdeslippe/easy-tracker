// Vitest.
import { describe, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React testing library.
import { render } from '@testing-library/react';

// Pages.
import { HomePage } from '@website/page/HomePage/HomePage';

// Test cases for rendering the home page component.
describe('Home Page component rendering', () => {
	it('Renders the home page component without crashing', () => {
		// Render the component.
		render(
			<BrowserRouter>
				<Routes>
					<Route
						path='/'
						element={<HomePage />}
					/>
				</Routes>
			</BrowserRouter>
		);
	});
});
