// React.
import { useEffect, useState } from 'react';

// React router.
import { Navigate } from 'react-router';

// Services.
import { logout } from '@website/feature/auth/service';

// Custom.
import { LoadingOverlay } from '@website/common/component/display';

/**
 * A logout page component.
 *
 * @returns The logout page.
 */
export function LogoutPage(): JSX.Element {
	const [error, setError] = useState<boolean>(false);
	const [completed, setCompleted] = useState<boolean>(false);

	useEffect(() => {
		logout(undefined)
			.then((response) => (response.status === 200 ? setCompleted(true) : setError(true)))
			.catch(() => setError(true));
	}, []);

	// If an error occurred.
	if (error) {
		// TODO: Redirect to the error page.
	}

	// If the user has been logged out.
	if (completed) {
		return <Navigate to='/' />;
	}

	// If the logout request has not completed.
	return <LoadingOverlay />;
}