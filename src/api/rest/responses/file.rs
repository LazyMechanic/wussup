use crate::api::rest::prelude::*;

#[derive(serde::Serialize, Debug)]
pub struct GetFiles {
    pub files: Vec<api_models::file::File>,
}

impl GetFiles {
    pub fn new(files: Vec<api_models::file::File>) -> Self {
        Self { files }
    }
}

#[derive(serde::Serialize, Debug)]
pub struct UploadFile {
    pub files: Vec<api_models::file::File>,
}

impl UploadFile {
    pub fn new(files: Vec<api_models::file::File>) -> Self {
        Self { files }
    }
}
