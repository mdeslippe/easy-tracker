// React.
import { Suspense, lazy } from 'react';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React Query.
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Hooks.
import { useAuthenticatedUser } from '@website/feature/auth/hook';

// Custom.
import { LoadingOverlay } from '@website/common/component/display';

// Pages.
const LandingPage = lazy(() =>
	import('@website/page').then((module) => ({ default: module.LandingPage }))
);
const HomePage = lazy(() =>
	import('@website/page').then((module) => ({ default: module.HomePage }))
);
const LoginPage = lazy(() =>
	import('@website/page').then((module) => ({ default: module.LoginPage }))
);
const SignUpPage = lazy(() =>
	import('@website/page').then((module) => ({ default: module.SignUpPage }))
);

// Create a react query client.
const queryClient = new QueryClient();

/**
 * The main application component.
 *
 * @returns The main application component.
 */
function App(): JSX.Element {
	return (
		<Suspense fallback={<LoadingOverlay />}>
			<QueryClientProvider client={queryClient}>
				<Router />
				<ReactQueryDevtools initialIsOpen={false} />
			</QueryClientProvider>
		</Suspense>
	);
}

/**
 * The router for the application.
 *
 * @returns The router.
 */
function Router(): JSX.Element {
	const { isAuthenticated } = useAuthenticatedUser();

	return (
		<BrowserRouter>
			<Routes>
				<Route
					path='/'
					Component={isAuthenticated ? HomePage : LandingPage}
				/>
				<Route
					path='/signup'
					Component={SignUpPage}
				/>
				<Route
					path='/login'
					Component={LoginPage}
				/>
			</Routes>
		</BrowserRouter>
	);
}

export default App;
