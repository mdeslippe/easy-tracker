// React.
import { JSX, Fragment, useEffect, useState } from 'react';

// React router.
import { Navigate } from 'react-router';

// React hook form.
import { zodResolver } from '@hookform/resolvers/zod';
import { UseFormSetError, useForm } from 'react-hook-form';

// Zod.
import { z } from 'zod';

// Models.
import { UpdateUserRequestDataSchema } from '@website/feature/user/model';
import {
	ValidationErrorResponseData,
	ValidationErrorResponseDataSchema
} from '@website/common/model';

// Services.
import { updateUser } from '@website/feature/user/service';

// Context.
import { SnackType } from '@website/common/context';

// Hooks.
import { useSnackBar } from '@website/common/hook';
import {
	UseAuthenticatedUserInvalidatorResult,
	UseAuthenticationStatusInvalidatorResult,
	useAuthenticatedUser,
	useAuthenticatedUserInvalidator,
	useAuthenticationStatusInvalidator
} from '@website/feature/auth/hook';

// Custom.
import { ErrorBox } from '@website/common/component/display';
import { ImageSelector, InputField } from '@website/common/component/input';

// Utils.
import { capitalizeFirstLetter } from '@website/utility/string';
import { convertValidationErrorToMessage } from '@website/utility/validation';

// CSS.
import '@website/feature/user/component/UserInformationForm/userInformationForm.css';

/**
 * A schema that can be used to validate the user information form values.
 */
const UserInformationFormDataSchema = z
	.intersection(
		UpdateUserRequestDataSchema,
		z.object({
			confirmPassword: z.string().optional()
		})
	)
	.refine((data) => data.password === data.confirmPassword, {
		message: 'Passwords must match.',
		path: ['confirmPassword']
	});

/**
 * The user information form data values.
 */
type UserInformationFormData = z.infer<typeof UserInformationFormDataSchema>;

/**
 * A function that can be used to handle the user information form submission.
 *
 * @param values The values the user entered in the input fields.
 * @param setFieldError A function that can be used to set field errors.
 * @param invalidateAuthenticatedUser A function that can be used to invalidate the authenticated user data.
 * @param invalidateAuthenticationStatus A function that can be used to invalidate the authentication status data.
 * @param onSuccess A function that will be invoked if the user's information was successfully updated.
 * @returns A promise.
 */
async function handleUserInformationUpdate(
	values: UserInformationFormData,
	setFieldError: UseFormSetError<UserInformationFormData>,
	invalidateAuthenticatedUser: UseAuthenticatedUserInvalidatorResult,
	invalidateAuthenticationStatus: UseAuthenticationStatusInvalidatorResult,
	onSuccess: () => void
): Promise<void> {
	// Send a request to update the user's information.
	const response = await updateUser(values);

	// Handle the response.
	switch (response.status) {
		case 200:
			invalidateAuthenticatedUser();
			invalidateAuthenticationStatus();
			onSuccess();
			return;
		case 400:
			// Parse the validation error response.
			const errors: ValidationErrorResponseData =
				await ValidationErrorResponseDataSchema.parseAsync(response.data);

			// Update the field errors.
			for (let [key, value] of Object.entries(errors)) {
				if (Object.prototype.hasOwnProperty.call(values, key)) {
					setFieldError(key as keyof UserInformationFormData, {
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
 * A user information form that can be used to modify information about a user.
 *
 * @returns The user information form.
 */
export function UserInformationForm(): JSX.Element {
	const snackbar = useSnackBar();
	const invalidateAuthenticatedUser = useAuthenticatedUserInvalidator();
	const invalidateAuthenticationStatus = useAuthenticationStatusInvalidator();
	const [errorMessage, setErrorMessage] = useState<string | null>(null);
	const [showAvatarSelector, setShowAvatarSelector] = useState<boolean>(false);
	const {
		isLoading: isUserLoading,
		isInitialLoading: isInitialUserLoading,
		isAuthenticated: isUserAuthenticated,
		user
	} = useAuthenticatedUser();
	const {
		register,
		handleSubmit,
		reset,
		setError,
		setValue,
		getValues,
		formState: { errors }
	} = useForm<UserInformationFormData>({
		resolver: async (values, context, options) => {
			// Create a shallow copy of the values so that we do not modify the original object.
			let valuesCopy = { ...values };

			// All empty input fields must be converted to undefined before validation.
			for (const key of Object.keys(valuesCopy)) {
				if (valuesCopy[key as keyof UserInformationFormData] === '') {
					valuesCopy[key as keyof UserInformationFormData] = undefined;
				}
			}

			// Perform validation and return the result.
			return await zodResolver(UserInformationFormDataSchema)(valuesCopy, context, options);
		}
	});

	// Populate the default field values after the user data is loaded for the first time.
	useEffect(() => {
		if (user !== null) {
			reset({
				profilePictureUrl: user.profilePictureUrl,
				username: user.username,
				email: user.email,
				password: '',
				confirmPassword: ''
			});
		}
	}, [user === null]);

	// If the user is not authenticated.
	if (!isUserLoading && !isUserAuthenticated) {
		return <Navigate to='/login' />;
	}

	// Define a function that will be invoked if the user's information was successfully updated.
	const onSuccessfulUpdate = () => {
		snackbar.addSnack({
			type: SnackType.SUCCESS,
			duration: 5000,
			message: 'Successfully updated information.'
		});
	};

	return (
		<Fragment>
			<ImageSelector
				id='avatar'
				title='Select an Avatar'
				open={showAvatarSelector}
				onClose={() => setShowAvatarSelector(false)}
				onSelect={(url) => {
					setValue('profilePictureUrl', url);
					setShowAvatarSelector(false);
				}}
			/>
			<form
				id='user-information-form'
				onSubmit={handleSubmit((values) => {
					handleUserInformationUpdate(
						values,
						setError,
						invalidateAuthenticatedUser,
						invalidateAuthenticationStatus,
						onSuccessfulUpdate
					).catch(() => {
						snackbar.addSnack({
							type: SnackType.ERROR,
							duration: 5000,
							message: 'An unexpected error has occurred.'
						});
					});
				})}
			>
				{errorMessage !== null && (
					<ErrorBox
						className='form-error-message'
						message={errorMessage}
						onClose={() => setErrorMessage(null)}
					/>
				)}
				<section>
					<h2>Avatar Settings</h2>
					<div className='avatar-input-container'>
						<img
							src={getValues('profilePictureUrl')}
							alt='Avatar'
						/>
						<div className='avatar-control-container'>
							<p>Upload a new avatar</p>
							<button
								className='primary-button small-button'
								type='button'
								title='Click to edit your avatar'
								onClick={() => setShowAvatarSelector(true)}
							>
								Choose picture...
							</button>
							{errors.profilePictureUrl && (
								<span className='input-error-message'>
									{errors.profilePictureUrl?.message?.toString()}
								</span>
							)}
						</div>
					</div>
				</section>
				<section>
					<h2>Account Settings</h2>
					<InputField
						label='Username'
						type='text'
						{...register('username')}
						error={errors.username?.message?.toString()}
					/>
					<InputField
						label='Email'
						type='email'
						{...register('email')}
						error={errors.email?.message?.toString()}
					/>
					<InputField
						label='Password'
						type='password'
						{...register('password')}
						error={errors.password?.message?.toString()}
					/>
					<InputField
						label='Confirm password'
						type='password'
						{...register('confirmPassword')}
						error={errors.confirmPassword?.message?.toString()}
					/>
				</section>
				<div className='form-button-container'>
					<input
						className='medium-button primary-button'
						type='submit'
						value='Save'
						disabled={isInitialUserLoading || user === null}
					/>
					<input
						className='medium-button secondary-button'
						type='reset'
						value='Reset'
						disabled={isInitialUserLoading || user === null}
						onClick={(event) => {
							// Prevent the default handler.
							event.preventDefault();

							// Remove the error message.
							setErrorMessage(null);

							// Reset the form.
							if (user === null) {
								reset();
							} else {
								reset({
									profilePictureUrl: user.profilePictureUrl,
									username: user.username,
									email: user.email,
									password: '',
									confirmPassword: ''
								});
							}
						}}
					/>
				</div>
			</form>
		</Fragment>
	);
}
