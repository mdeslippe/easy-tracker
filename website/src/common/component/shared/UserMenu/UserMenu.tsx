import { Navigate } from 'react-router';

// Hooks.
import { useAuthenticatedUser } from '@website/feature/auth/hook';

// Custom.
import { Skeleton } from '@website/common/component/display';
import { DropDownMenu } from '@website/common/component/layout';
import { NavigationButton } from '@website/common/component/navigation';

// CSS.
import '@website/common/component/shared/UserMenu/userMenu.css';

/**
 * A user menu component.
 *
 * **Do not use this component if the user is not authenticated, it will redirect them
 * to the error page.**
 *
 * @returns The user menu.
 */
export function UserMenu(): JSX.Element {
	const { isLoading, isInitialLoading, isError, isAuthenticated, user } = useAuthenticatedUser();

	// If an error occurred while getting the user's data.
	if (isError) {
		return <Navigate to='/error' />;
	}

	// If the user is not authenticated.
	if (!isLoading && (!isAuthenticated || user === null)) {
		return <Navigate to='/error' />;
	}

	// If the user's data is being loaded for the first time, show a skeleton.
	if (isLoading && isInitialLoading) {
		return <Skeleton className='user-menu-skeleton' />;
	}

	return (
		<DropDownMenu
			accessibilityLabel='User Menu'
			align='right'
			buttonContent={
				<img
					className='user-menu-picture no-select'
					src={user?.profilePictureUrl}
					alt={`@${user?.username}`}
				/>
			}
		>
			<div className='user-menu-content-container'>
				<NavigationButton to='/logout'>Logout</NavigationButton>
			</div>
		</DropDownMenu>
	);
}
