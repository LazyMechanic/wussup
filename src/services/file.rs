use crate::config;
use crate::models::file::*;
use crate::repos::prelude::*;
use crate::services::local_prelude::*;

use futures::prelude::*;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub struct FileService {
    cfg: config::File,
    db: DbPool,
}

impl FileService {
    pub fn new(cfg: config::File, db: DbPool) -> FileService {
        FileService { cfg, db }
    }

    pub async fn get_files(&self) -> Result<Vec<File>, ServiceError> {
        let rows = self.db.file_repo().get_files().await?;
        Ok(rows)
    }

    pub async fn has_file<S>(&self, path: S) -> Result<bool, ServiceError>
    where
        S: AsRef<str>,
    {
        let has = self.db.file_repo().has_file(path).await?;
        Ok(has)
    }

    pub async fn upload<S, B, E>(
        &self,
        mut stream: S,
        path: String,
    ) -> Result<Vec<File>, ServiceError>
    where
        S: futures::Stream<Item = Result<B, E>> + Unpin,
        B: bytes::Buf,
        E: std::error::Error + Send + Sync + 'static,
    {
        if self.has_file(&path).await? {
            return Err(ServiceError::CommonError(anyhow!("file exists")));
        }

        let fs_path = Path::new(&self.cfg.base_path).join(&path);
        let mut file = fs::File::create(fs_path)
            .await
            .map_err(|err| ServiceError::CommonError(err.into()))?;

        // Write stream to file
        while let Some(mut buf) = stream
            .try_next()
            .await
            .map_err(|err| ServiceError::CommonError(err.into()))?
        {
            // While data in buf exists
            while buf.has_remaining() {
                file.write_buf(&mut buf)
                    .await
                    .map_err(|err| ServiceError::CommonError(err.into()))?;
            }
        }

        // Save file to repo
        let f = File {
            id: Uuid::new_v4(),
            path,
        };
        self.db.file_repo().add_file(f).await?;

        // Get actual files
        let files = self.db.file_repo().get_files().await?;

        Ok(files)
    }
}
