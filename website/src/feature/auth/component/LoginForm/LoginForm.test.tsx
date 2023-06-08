// Vitest.
import { describe, expect, it } from 'vitest';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React testing library.
import { render } from '@testing-library/react';

// React query.
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Custom.
import { LoginForm } from '@website/feature/auth/component/LoginForm/LoginForm';

// Test cases for rendering the login form component.
describe('Login Form component rendering', () => {
	it('Renders the login form without crashing', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<LoginForm />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);
	});

	it('Renders all of the input fields inside of the login form', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		const result = render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<LoginForm />}
						/>
					</Routes>
				</BrowserRouter>
			</QueryClientProvider>
		);

		// Make sure the input fields were rendered.
		expect(result.container.querySelector('#username')).toBeDefined();
		expect(result.container.querySelector('#password')).toBeDefined();
	});

	it('Renders buttons to submit and reset the login form', () => {
		// Create a react query client.
		const queryClient = new QueryClient();

		// Render the component.
		const result = render(
			<QueryClientProvider client={queryClient}>
				<BrowserRouter>
					<Routes>
						<Route
							path='/'
							element={<LoginForm />}
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
