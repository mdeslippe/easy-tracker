use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// A user struct.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct User {
    /// The user's unique identifier.
    pub(crate) id: u64,

    /// The date and time the user's account was created at.
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) account_created_at: OffsetDateTime,

    /// The date and time the user's password was last reset at.
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) password_reset_at: OffsetDateTime,

    /// A url to the user's profile picture.
    pub(crate) profile_picture_url: String,

    /// The user's username.
    pub(crate) username: String,

    /// The user's password.
    pub(crate) password: String,

    /// The user's email address.
    pub(crate) email: String,

    /// If the user has verified their email address or not.
    pub(crate) email_is_verified: bool,

    /// If the user is required to reset their password before they can login.
    pub(crate) password_reset_is_required: bool,

    /// If the user's account has been locked.
    pub(crate) account_is_locked: bool,

    /// If the user's account has been banned.
    pub(crate) account_is_banned: bool,
}

/// A Default implementation for the User struct.
impl Default for User {
    fn default() -> Self {
        return User {
            id: 0,
            account_created_at: OffsetDateTime::now_utc(),
            password_reset_at: OffsetDateTime::now_utc(),
            profile_picture_url: String::from(""),
            username: String::from(""),
            password: String::from(""),
            email: String::from(""),
            email_is_verified: false,
            password_reset_is_required: false,
            account_is_locked: false,
            account_is_banned: false,
        };
    }
}

/// A PartialEq implementation for the User struct.
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.account_created_at.date() == other.account_created_at.date()
            && self.account_created_at.hour() == other.account_created_at.hour()
            && self.account_created_at.minute() == other.account_created_at.minute()
            && self.account_created_at.second() == other.account_created_at.second()
            && self.password_reset_at.date() == other.password_reset_at.date()
            && self.password_reset_at.hour() == other.password_reset_at.hour()
            && self.password_reset_at.minute() == other.password_reset_at.minute()
            && self.password_reset_at.second() == other.password_reset_at.second()
            && self.profile_picture_url == other.profile_picture_url
            && self.username == other.username
            && self.password == other.password
            && self.email == other.email
            && self.email_is_verified == other.email_is_verified
            && self.password_reset_is_required == other.password_reset_is_required
            && self.account_is_locked == other.account_is_locked
            && self.account_is_banned == other.account_is_banned
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}
