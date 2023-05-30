// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React Query.
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Pages.
import { LandingPage, LoginPage, SignUpPage } from '@website/page';

// Create a react query client.
const queryClient = new QueryClient();

/**
 * The main application component.
 *
 * @returns The main application component.
 */
function App(): JSX.Element {
	return (
		<QueryClientProvider client={queryClient}>
			<BrowserRouter>
				<Routes>
					<Route
						path='/'
						element={<LandingPage />}
					/>
					<Route
						path='/signup'
						element={<SignUpPage />}
					/>
					<Route
						path='/login'
						element={<LoginPage />}
					/>
				</Routes>
			</BrowserRouter>
			<ReactQueryDevtools initialIsOpen={false} />
		</QueryClientProvider>
	);
}

export default App;
