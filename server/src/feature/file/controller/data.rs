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

/// A get file request query parameter struct.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GetFileRequestParams {
    /// If the raw file data should be returned.
    pub(super) raw: Option<bool>,
}

/// An update file request body struct.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UpdateFileRequestBody {
    /// The file's mime type.
    pub(super) mime_type: Option<String>,

    /// The file's name.
    pub(super) name: Option<String>,

    /// The file's raw data.
    pub(super) data: Option<Vec<u8>>,
}

/// An implementation for the UpdateFileRequestBody struct.
impl UpdateFileRequestBody {
    /// # Description
    ///
    /// Apply the changes in the update file request body to a file.
    ///
    /// # Arguments
    ///
    /// `file` - The file the changes will be applied to.
    pub(super) fn apply(&self, file: &mut File) {
        // If the file's mime type is being updated.
        if let Some(mime_type) = &self.mime_type {
            file.mime_type = mime_type.clone();
        }

        // If the file's name is being updated.
        if let Some(name) = &self.name {
            file.name = name.clone();
        }

        // If the file's data is being updated.
        if let Some(data) = &self.data {
            file.data = data.clone();
        }
    }
}
