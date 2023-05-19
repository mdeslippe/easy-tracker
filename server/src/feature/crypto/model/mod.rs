use serde::{Deserialize, Serialize};

/// A struct containing claims for users tokens.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserClaims {
    /// The id of the user the token belongs to.
    pub(crate) id: u64,

    /// The date and time the token expires (UTC).
    pub(crate) exp: i64,

    /// The date and time the token was issued at (UTC).
    pub(crate) iat: i64,

    /// The date and time the user last reset their password when the token was generated (UTC).
    /// If this does not align with the last password reset date and time in the user's persistent
    /// storage record, the token is not longer valid.
    pub(crate) password_last_reset: i64,
}
