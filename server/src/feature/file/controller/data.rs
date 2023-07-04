use crate::feature::file::model::File;
use serde::{Deserialize, Serialize};

/// A create file request body struct.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateFileRequestBody {
    /// The file's mime type.
    pub(super) mime_type: String,

    /// The file's name.
    pub(super) name: String,

    /// The file's raw data.
    pub(super) data: Vec<u8>,
}

/// An Into<File> implementation for the CreateFileRequestBody struct.
impl Into<File> for CreateFileRequestBody {
    fn into(self) -> File {
        return File {
            mime_type: self.mime_type,
            name: self.name,
            data: self.data,
            ..Default::default()
        };
    }
}
