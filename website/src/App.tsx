// React.
import { LazyExoticComponent, Suspense, lazy } from 'react';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React Query.
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Hooks.
import { useAuthenticationStatus } from '@website/feature/auth/hook';

// Custom.
import { LoadingOverlay } from '@website/common/component/display';

// Pages.
const LandingPage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page').then((module) => ({ default: module.LandingPage }))
);
const HomePage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page').then((module) => ({ default: module.HomePage }))
);
const LoginPage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page').then((module) => ({ default: module.LoginPage }))
);
const SignUpPage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
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
	const { isLoading, isInitialLoading, isAuthenticated } = useAuthenticationStatus();

	// If the authenticated user's data is being loaded for the first time, show the loading overlay.
	if (isLoading && isInitialLoading) return <LoadingOverlay />;

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
