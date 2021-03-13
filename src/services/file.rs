use crate::config;
use crate::models::file::*;
use crate::services::local_prelude::*;
use crate::services::utils;

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

    pub async fn get_file(&self, id: Uuid) -> Result<File, ServiceError> {
        let row = self.db.file_repo().get_file(id).await?;
        Ok(row)
    }

    fn has_fs_file<S1, S2, S3>(
        &self,
        platform: S1,
        build: S2,
        version: S3,
    ) -> Result<bool, ServiceError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let has_fs = Path::new(&self.cfg.base_path)
            .join(utils::format_file_name(&platform, &build, &version))
            .exists();

        Ok(has_fs)
    }

    async fn has_repo_file<S1, S2, S3>(
        &self,
        platform: S1,
        build: S2,
        version: S3,
    ) -> Result<bool, ServiceError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let has_db = self
            .db
            .file_repo()
            .has_file(&platform, &build, &version)
            .await?;

        Ok(has_db)
    }

    pub async fn has_file<S1, S2, S3>(
        &self,
        platform: S1,
        build: S2,
        version: S3,
    ) -> Result<bool, ServiceError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let has_db = self.has_repo_file(&platform, &build, &version).await?;

        let has_fs = self.has_fs_file(&platform, &build, &version)?;

        Ok(has_db || has_fs)
    }

    pub async fn upload<S, B, E>(
        &self,
        mut stream: S,
        platform: String,
        build: String,
        version: String,
    ) -> Result<Vec<File>, ServiceError>
    where
        S: futures::Stream<Item = Result<B, E>> + Unpin,
        B: bytes::Buf,
        E: std::error::Error + Send + Sync + 'static,
    {
        if self.has_file(&platform, &build, &version).await? {
            return Err(ServiceError::CommonError(anyhow!("file exists")));
        }

        let fs_path = Path::new(&self.cfg.base_path)
            .join(utils::format_file_name(&platform, &build, &version));
        let mut fs_file = fs::File::create(fs_path)
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
                fs_file
                    .write_buf(&mut buf)
                    .await
                    .map_err(|err| ServiceError::CommonError(err.into()))?;
            }
        }

        // Save file to repo
        let f = NewFile {
            platform,
            build,
            version,
        };
        self.db.file_repo().create_file(f).await?;

        // Get actual files
        let files = self.db.file_repo().get_files().await?;

        Ok(files)
    }

    pub async fn download(
        &self,
        platform: String,
        build: String,
        version: String,
    ) -> Result<impl futures::Stream<Item = Result<bytes::BytesMut, std::io::Error>>, ServiceError>
    {
        if !self.has_file(&platform, &build, &version).await? {
            return Err(ServiceError::CommonError(anyhow!("file not exists")));
        }

        let fs_path = Path::new(&self.cfg.base_path)
            .join(utils::format_file_name(&platform, &build, &version));
        let fs_file = fs::File::open(fs_path)
            .await
            .map_err(|err| ServiceError::CommonError(err.into()))?;

        let codec =
            tokio_util::codec::FramedRead::new(fs_file, tokio_util::codec::BytesCodec::new());

        Ok(codec)
    }

    pub async fn delete(
        &self,
        platform: String,
        build: String,
        version: String,
    ) -> Result<Vec<File>, ServiceError> {
        if !self.has_file(&platform, &build, &version).await? {
            return Err(ServiceError::CommonError(anyhow!("file not exists")));
        }

        // Unwrap, because already check
        let file = self
            .db
            .file_repo()
            .find_file(&platform, &build, &version)
            .await?
            .unwrap();

        // Check if file using in settings
        if self.db.settings_repo().has_file(file.id).await? {
            return Err(ServiceError::CommonError(anyhow!(
                "file using in settings, update it before deleting file"
            )));
        }

        let fs_path = Path::new(&self.cfg.base_path)
            .join(utils::format_file_name(&platform, &build, &version));

        // Delete file from fs
        std::fs::remove_file(fs_path).map_err(|err| ServiceError::CommonError(err.into()))?;

        // Delete file from repo
        self.db.file_repo().delete_file(file.id).await?;

        // Get actual files
        let files = self.db.file_repo().get_files().await?;

        Ok(files)
    }
}
