// React.
import { useState, Dispatch, SetStateAction } from 'react';

// React hook form.
import { zodResolver } from '@hookform/resolvers/zod';
import { FieldValues, useForm } from 'react-hook-form';

// Models.
import { LoginRequestDataSchema } from '@website/feature/auth/model';

// Service.
import { login } from '@website/feature/auth/service';

// Custom.
import { ErrorBox } from '@website/common/component/display';
import { InputField } from '@website/common/component/input';

// CSS.
import '@website/feature/auth/component/LoginForm/loginForm.css';

/**
 * A schema that can be used to validate the login form values.
 */
const LoginFormSchema = LoginRequestDataSchema;

/**
 * A function that can be used to handle the login form submission.
 *
 * @param values The values the user entered in the input fields.
 * @param setError A function that can be used to set an error message.
 * @returns A promise.
 */
async function handleLogin(values: FieldValues, setError: Dispatch<SetStateAction<string | null>>) {
	// Send a request to authenticate the user.
	const response = await login(await LoginRequestDataSchema.parseAsync(values));

	// Handle the response.
	switch (response.status) {
		case 200:
			// TODO: Fetch user data and store it with react query.
			// TODO: Redirect the user to the dashboard.

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
	const [errorMessage, setErrorMessage] = useState<string | null>(null);
	const {
		register,
		handleSubmit,
		reset,
		formState: { errors }
	} = useForm({
		resolver: zodResolver(LoginFormSchema)
	});

	return (
		<form
			id='login-form'
			onSubmit={handleSubmit((values) =>
				handleLogin(values, setErrorMessage).catch(() =>
					setErrorMessage('An unexpected error has occurred.')
				)
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
