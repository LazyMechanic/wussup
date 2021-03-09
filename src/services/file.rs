use crate::models::file::*;
use crate::repos::prelude::*;
use crate::services::local_prelude::*;

pub struct FileService {
    db: DbPool,
}

impl FileService {
    pub fn new(db: DbPool) -> FileService {
        FileService { db }
    }

    pub async fn get_files(&self) -> Result<Vec<File>, ServiceError> {
        let rows = self.db.file_repo().get_files().await?;
        Ok(rows)
    }
}
