use crate::api::rest::prelude::*;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct File {
    pub id: Uuid,
    pub path: String,
}

impl From<db_models::file::File> for File {
    fn from(f: db_models::file::File) -> Self {
        Self {
            id: f.id,
            path: f.path,
        }
    }
}
