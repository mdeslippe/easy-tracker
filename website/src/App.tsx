// React router.
import { BrowserRouter, Route, Routes } from 'react-router-dom';

/**
 * The main application component.
 *
 * @returns The main application component.
 */
function App(): JSX.Element {
	return (
		<BrowserRouter>
			<Routes>
				<Route path='/' />
			</Routes>
		</BrowserRouter>
	);
}

export default App;
