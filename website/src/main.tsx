// React.
import React from 'react';
import ReactDOM from 'react-dom/client';

// Custom.
import App from '@website/App.tsx';

// CSS.
import './index.css';

// Mount the application.
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<App />
	</React.StrictMode>
);
