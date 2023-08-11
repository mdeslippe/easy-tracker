// React.
import React from 'react';
import ReactDOM from 'react-dom/client';

// Custom.
import App from '@website/App.tsx';

// CSS.
import '@website/index.css';
import '@website/reset.css';
import 'react-image-crop/dist/ReactCrop.css';

// Mount the application.
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<App />
	</React.StrictMode>
);
