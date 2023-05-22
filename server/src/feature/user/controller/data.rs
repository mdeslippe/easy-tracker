use crate::feature::user::model::User;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// A create user request body struct.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateUserRequestBody {
    /// The user's username.
    pub(super) username: String,

    /// The user's password.
    pub(super) password: String,

    /// The user's email address.
    pub(super) email: String,
}

/// An Into<User> implementation for the CreateUserRequestBody struct.
impl Into<User> for CreateUserRequestBody {
    fn into(self) -> User {
        return User {
            username: self.username,
            password: self.password,
            email: self.email,
            ..Default::default()
        };
    }
}

/// A get user response body struct.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GetUserResponseBody {
    /// The user's unique identifier.
    pub(crate) id: u64,

    /// The date and time the user's account was created at.
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) account_created_at: OffsetDateTime,

    /// A url to the user's profile picture.
    pub(crate) profile_picture_url: String,

    /// The user's username.
    pub(crate) username: String,
}

/// An Into<GetUserResponseBody> implementation for the User struct.
impl Into<GetUserResponseBody> for User {
    fn into(self) -> GetUserResponseBody {
        return GetUserResponseBody {
            id: self.id,
            account_created_at: self.account_created_at,
            profile_picture_url: self.profile_picture_url,
            username: self.username,
        };
    }
}

/// An update user request body struct.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UpdateUserRequestBody {
    /// A url to the user's profile picture.
    pub(super) profile_picture_url: Option<String>,

    /// The user's username.
    pub(super) username: Option<String>,

    /// The user's password.
    pub(super) password: Option<String>,

    /// The user's email address.
    pub(super) email: Option<String>,
}

/// An implementation for the UpdateUserRequestBody struct.
impl UpdateUserRequestBody {
    /// # Description
    ///
    /// Apply the changes in a the request body to a user.
    ///
    /// # Arguments
    ///
    /// `user` - The user the changes will be applied to.
    pub(super) fn apply(&self, user: &mut User) {
        // If the user is attempting to update their profile picture.
        if let Some(profile_picture_url) = &self.profile_picture_url {
            user.profile_picture_url = profile_picture_url.clone();
        }

        // If the user is attempting to update their username.
        if let Some(username) = &self.username {
            user.username = username.clone();
        }

        // If the user is attempting to update their password.
        if let Some(password) = &self.password {
            user.password = password.clone();
        }

        // If the user is attempting to update their email address.
        if let Some(email) = &self.email {
            user.email = email.clone();
        }
    }
}
