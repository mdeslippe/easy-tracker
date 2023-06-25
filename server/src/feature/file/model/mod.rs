use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use validator::Validate;

/// A file struct.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub(crate) struct File {
    /// The file's unique identifier.
    pub(crate) id: u64,

    /// The unique identifier of the user that the file belongs to.
    pub(crate) user_id: u64,

    /// The date and time the file was created at.
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) file_created_at: OffsetDateTime,

    /// The file's mime type.
    #[validate(non_control_character, length(min = 1, max = 256))]
    pub(crate) mime_type: String,

    /// The file's name.
    #[validate(non_control_character, length(min = 1, max = 256))]
    pub(crate) name: String,

    /// The file's raw data.
    pub(crate) data: Vec<u8>,
}

/// A Default implementation for the File struct.
impl Default for File {
    fn default() -> Self {
        return File {
            id: 0,
            user_id: 0,
            file_created_at: OffsetDateTime::now_utc(),
            mime_type: String::from(""),
            name: String::from(""),
            data: vec![],
        };
    }
}

/// A PartialEq implementation for the File struct.
impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.user_id == other.user_id
            && self.file_created_at.date() == other.file_created_at.date()
            && self.file_created_at.hour() == other.file_created_at.hour()
            && self.file_created_at.minute() == other.file_created_at.minute()
            && self.file_created_at.second() == other.file_created_at.second()
            && self.mime_type == other.mime_type
            && self.name == other.name
            && self.data == other.data;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}
