// React.
import { JSX, useState } from 'react';

// React router.
import { NavigateFunction, useNavigate } from 'react-router';

// React hook form.
import { zodResolver } from '@hookform/resolvers/zod';
import { UseFormSetError, useForm } from 'react-hook-form';

// Zod.
import { z } from 'zod';

// Models.
import { CreateUserRequestDataSchema } from '@website/feature/user/model';
import {
	ValidationErrorResponseData,
	ValidationErrorResponseDataSchema
} from '@website/common/model';

// Services.
import { createUser } from '@website/feature/user/service';

// Hook.
import {
	UseAuthenticatedUserResetterResult,
	UseAuthenticationStatusResetterResult,
	useAuthenticatedUserResetter,
	useAuthenticationStatusResetter
} from '@website/feature/auth/hook';

// Custom.
import { InputField } from '@website/common/component/input';
import { ErrorBox } from '@website/common/component/display';

// Utils.
import { capitalizeFirstLetter } from '@website/utility/string';
import { convertValidationErrorToMessage } from '@website/utility/validation';

// CSS.
import '@website/feature/user/component/SignUpForm/signUpForm.css';

/**
 * A schema that can be used to validate the sign up form values.
 */
const SignUpFormDataSchema = z
	.intersection(
		CreateUserRequestDataSchema,
		z.object({
			confirmPassword: z.string()
		})
	)
	.refine((data) => data.password === data.confirmPassword, {
		message: 'Passwords must match.',
		path: ['confirmPassword']
	});

/**
 * The sign up form data values.
 */
type SignUpFormData = z.infer<typeof SignUpFormDataSchema>;

/**
 * A function that can be used to handle the sign up form submission.
 *
 * @param values The values the user entered in the input fields.
 * @param setFieldError A function that can be used to set field errors.
 * @param navigate A function that can be used to navigate the user to a different route.
 * @param resetAuthenticatedUser A function that can be used to reset the authenticated user data.
 * @param resetAuthenticationStatus A function that can be used to reset the authentication status data.
 * @returns A promise.
 */
async function handleSignUp(
	values: SignUpFormData,
	setFieldError: UseFormSetError<SignUpFormData>,
	navigate: NavigateFunction,
	resetAuthenticatedUser: UseAuthenticatedUserResetterResult,
	resetAuthenticationStatus: UseAuthenticationStatusResetterResult
): Promise<void> {
	// Send a request to sign the user up.
	const response = await createUser(values);

	// Handle the response.
	switch (response.status) {
		case 200:
			resetAuthenticatedUser();
			resetAuthenticationStatus();
			navigate('/');
			return;
		case 400:
			// Parse the validation error response.
			const errors: ValidationErrorResponseData =
				await ValidationErrorResponseDataSchema.parseAsync(response.data);

			// Update the field errors.
			for (let [key, value] of Object.entries(errors)) {
				if (Object.prototype.hasOwnProperty.call(values, key)) {
					setFieldError(key as keyof SignUpFormData, {
						// There could be multiple validation errors, but we only want to display one at
						// a time - to keep things simple, we just take the first error in the array.
						message: convertValidationErrorToMessage(capitalizeFirstLetter(key), value[0])
					});
				}
			}
			return;
		case 500:
			throw Error();
		default:
			throw Error();
	}
}

/**
 * A sign up form component that can be used to create new users.
 *
 * @returns The sign up form.
 */
export function SignUpForm(): JSX.Element {
	const navigate = useNavigate();
	const resetAuthenticatedUser = useAuthenticatedUserResetter();
	const resetAuthenticationStatus = useAuthenticationStatusResetter();
	const [errorMessage, setErrorMessage] = useState<string | null>(null);
	const {
		register,
		handleSubmit,
		reset,
		setError,
		formState: { errors }
	} = useForm<SignUpFormData>({
		resolver: zodResolver(SignUpFormDataSchema)
	});

	return (
		<form
			id='sign-up-form'
			onSubmit={handleSubmit((values) =>
				handleSignUp(
					values,
					setError,
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
				label='Email'
				type='email'
				{...register('email')}
				error={errors.email?.message?.toString()}
			/>
			<InputField
				required
				label='Password'
				type='password'
				{...register('password')}
				error={errors.password?.message?.toString()}
			/>
			<InputField
				required
				label='Confirm password'
				type='password'
				{...register('confirmPassword')}
				error={errors.confirmPassword?.message?.toString()}
			/>
			<div className='form-button-container'>
				<input
					className='medium-button primary-button'
					type='submit'
					value='Sign Up'
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
