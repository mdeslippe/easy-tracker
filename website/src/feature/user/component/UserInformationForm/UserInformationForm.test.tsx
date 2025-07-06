// Vitest.
import { describe, expect, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router';

// React testing library.
import { render } from '@testing-library/react';

// React query.
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Custom/
import { UserInformationForm } from '@website/feature/user/component/UserInformationForm/UserInformationForm';

// Test cases for rendering the user information form component.
describe('User Information Form component rendering', () => {
	it('Renders the user information form without crashing', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<UserInformationForm />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);
	});

	it('Renders all of the input fields inside of the user information form', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		const result = render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<UserInformationForm />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);

		// Make sure the input fields were rendered.
		expect(result.container.querySelector('#username')).toBeDefined();
		expect(result.container.querySelector('#email')).toBeDefined();
		expect(result.container.querySelector('#password')).toBeDefined();
		expect(result.container.querySelector('#confirmPassword')).toBeDefined();
	});

	it('Renders buttons to submit and reset the user information form', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		const result = render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<UserInformationForm />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);

		// Make sure the buttons to submit and reset the form were rendered.
		expect(result.container.querySelector('input[type="submit"]')).toBeDefined();
		expect(result.container.querySelector('input[type="reset"]')).toBeDefined();
	});
});
