// React.
import { LazyExoticComponent, Suspense, lazy } from 'react';

// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

// React Query.
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

// Custom.
import { LoadingOverlay } from '@website/common/component/display';
import {
	AuthenticatedRoute,
	ConditionalAuthenticationRoute,
	UnauthenticatedRoute
} from '@website/common/component/navigation';

// Pages.
const LandingPage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page/LandingPage').then((module) => ({ default: module.LandingPage }))
);
const HomePage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page/HomePage').then((module) => ({ default: module.HomePage }))
);
const LoginPage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page/LoginPage').then((module) => ({ default: module.LoginPage }))
);
const SignUpPage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page/SignUpPage').then((module) => ({ default: module.SignUpPage }))
);
const LogoutPage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page/LogoutPage').then((module) => ({ default: module.LogoutPage }))
);
const UserSettingsPage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page/UserSettingsPage').then((module) => ({
		default: module.UserSettingsPage
	}))
);
const ErrorPage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page/ErrorPage').then((module) => ({ default: module.ErrorPage }))
);
const NotFoundPage: LazyExoticComponent<() => JSX.Element> = lazy(() =>
	import('@website/page/NotFoundPage').then((module) => ({ default: module.NotFoundPage }))
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
	return (
		<BrowserRouter>
			<Routes>
				<Route
					path='/'
					Component={() => (
						<ConditionalAuthenticationRoute
							AuthenticatedComponent={HomePage}
							UnauthenticatedComponent={LandingPage}
						/>
					)}
				/>
				<Route
					path='/signup'
					Component={() => (
						<UnauthenticatedRoute
							redirectTo='/'
							Component={SignUpPage}
						/>
					)}
				/>
				<Route
					path='/login'
					Component={() => (
						<UnauthenticatedRoute
							redirectTo='/'
							Component={LoginPage}
						/>
					)}
				/>
				<Route
					path='/logout'
					Component={() => (
						<AuthenticatedRoute
							redirectTo='/'
							Component={LogoutPage}
						/>
					)}
				/>
				<Route
					path='/settings'
					Component={() => (
						<AuthenticatedRoute
							redirectTo='/login'
							Component={UserSettingsPage}
						/>
					)}
				/>
				<Route
					path='/error'
					Component={ErrorPage}
				/>
				<Route
					path='*'
					Component={NotFoundPage}
				/>
			</Routes>
		</BrowserRouter>
	);
}

export default App;
