use crate::api::rest::prelude::*;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct NewFile {
    pub platform: String,
    pub build: String,
    pub version: String,
}

impl From<NewFile> for db_models::file::NewFile {
    fn from(f: NewFile) -> Self {
        Self {
            platform: f.platform,
            build: f.build,
            version: f.version,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct File {
    pub id: Uuid,
    pub platform: String,
    pub build: String,
    pub version: String,
}

impl From<db_models::file::File> for File {
    fn from(f: db_models::file::File) -> Self {
        Self {
            id: f.id,
            platform: f.platform,
            build: f.build,
            version: f.version,
        }
    }
}

#[derive(serde::Serialize, Debug)]
pub struct GetFilesResponse {
    pub files: Vec<File>,
}

impl From<Vec<db_models::file::File>> for GetFilesResponse {
    fn from(f: Vec<db_models::file::File>) -> Self {
        Self {
            files: f.into_iter().map(|f| f.into()).collect(),
        }
    }
}

#[derive(serde::Serialize, Debug)]
pub struct UploadFileResponse {
    pub files: Vec<File>,
}

impl From<Vec<db_models::file::File>> for UploadFileResponse {
    fn from(f: Vec<db_models::file::File>) -> Self {
        Self {
            files: f.into_iter().map(|f| f.into()).collect(),
        }
    }
}
