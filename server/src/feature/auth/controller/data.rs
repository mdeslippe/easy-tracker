use serde::{Deserialize, Serialize};

/// A login request body struct.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LoginRequestBody {
    /// The user's username.
    pub(super) username: String,

    /// The user's password.
    pub(super) password: String,
}
