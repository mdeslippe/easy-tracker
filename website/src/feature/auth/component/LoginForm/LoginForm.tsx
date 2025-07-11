// React.
import { JSX, useState, Dispatch, SetStateAction } from 'react';

// React router.
import { NavigateFunction, useNavigate } from 'react-router';

// React hook form.
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';

// Zod.
import { z } from 'zod';

// Models.
import { LoginRequestDataSchema } from '@website/feature/auth/model';

// Service.
import { login } from '@website/feature/auth/service';

// Hook.
import {
	UseAuthenticatedUserResetterResult,
	UseAuthenticationStatusResetterResult,
	useAuthenticatedUserResetter,
	useAuthenticationStatusResetter
} from '@website/feature/auth/hook';

// Custom.
import { ErrorBox } from '@website/common/component/display';
import { InputField } from '@website/common/component/input';

// CSS.
import '@website/feature/auth/component/LoginForm/loginForm.css';

/**
 * A schema that can be used to validate the login form values.
 */
const LoginFormDataSchema = LoginRequestDataSchema;

/**
 * The login form data values.
 */
type LoginFormData = z.infer<typeof LoginFormDataSchema>;

/**
 * A function that can be used to handle the login form submission.
 *
 * @param values The values the user entered in the input fields.
 * @param setError A function that can be used to set an error message.
 * @param navigate A function that can be used to navigate the user to a different route.
 * @param resetAuthenticatedUser A function that can be used to reset the authenticated user data.
 * @param resetAuthenticationStatus A function that can be used to reset the authentication status data.
 * @returns A promise.
 */
async function handleLogin(
	values: LoginFormData,
	setError: Dispatch<SetStateAction<string | null>>,
	navigate: NavigateFunction,
	resetAuthenticatedUser: UseAuthenticatedUserResetterResult,
	resetAuthenticationStatus: UseAuthenticationStatusResetterResult
) {
	// Send a request to authenticate the user.
	const response = await login(values);

	// Handle the response.
	switch (response.status) {
		case 200:
			resetAuthenticatedUser();
			resetAuthenticationStatus();
			navigate('/');
			return;
		case 401:
			setError('Invalid username or password.');
			return;
		case 500:
			throw Error();
		default:
			throw Error();
	}
}

/**
 * A login form component that can be used to authenticate users.
 *
 * @returns The login form.
 */
export function LoginForm(): JSX.Element {
	const navigate = useNavigate();
	const resetAuthenticatedUser = useAuthenticatedUserResetter();
	const resetAuthenticationStatus = useAuthenticationStatusResetter();
	const [errorMessage, setErrorMessage] = useState<string | null>(null);
	const {
		register,
		handleSubmit,
		reset,
		formState: { errors }
	} = useForm<LoginFormData>({
		resolver: zodResolver(LoginFormDataSchema)
	});

	return (
		<form
			id='login-form'
			onSubmit={handleSubmit((values) =>
				handleLogin(
					values,
					setErrorMessage,
					navigate,
					resetAuthenticatedUser,
					resetAuthenticationStatus
				).catch(() => setErrorMessage('An unexpected error has occurred.'))
			)}
		>
			{errorMessage !== null && (
				<ErrorBox
					className='form-error-message'
					message={errorMessage}
					onClose={() => setErrorMessage(null)}
				/>
			)}
			<InputField
				required
				label='Username'
				type='text'
				{...register('username')}
				error={errors.username?.message?.toString()}
			/>
			<InputField
				required
				label='Password'
				type='password'
				{...register('password')}
				error={errors.password?.message?.toString()}
			/>
			<div className='form-button-container'>
				<input
					className='medium-button primary-button'
					type='submit'
					value='Login'
				/>
				<input
					className='medium-button secondary-button'
					type='reset'
					value='Reset'
					onClick={() => {
						reset();
						setErrorMessage(null);
					}}
				/>
			</div>
		</form>
	);
}
