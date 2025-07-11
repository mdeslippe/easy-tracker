// React.
import { JSX, useEffect, useState } from 'react';

// React router.
import { Navigate } from 'react-router';

// Services.
import { logout } from '@website/feature/auth/service';

// Hooks.
import {
	useAuthenticatedUserResetter,
	useAuthenticationStatusResetter
} from '@website/feature/auth/hook';

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
	const resetAuthenticatedUser = useAuthenticatedUserResetter();
	const resetAuthenticationStatus = useAuthenticationStatusResetter();

	useEffect(() => {
		logout(undefined)
			.then((response) => {
				// If the logout was successful.
				if (response.status === 200) {
					setCompleted(true);
					resetAuthenticatedUser();
					resetAuthenticationStatus();
				} else {
					setError(true);
				}
			})
			.catch(() => setError(true));
	}, []);

	// If an error occurred.
	if (error) {
		return <Navigate to='/error' />;
	}

	// If the user has been logged out.
	if (completed) {
		return <Navigate to='/' />;
	}

	// If the logout request has not completed.
	return <LoadingOverlay />;
}
