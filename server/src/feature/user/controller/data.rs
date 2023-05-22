use serde::{Deserialize, Serialize};

use crate::feature::user::model::User;

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
