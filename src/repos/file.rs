use crate::models::file::*;
use crate::repos::DbPool;
use crate::repos::RepoError;
use uuid::Uuid;

pub struct FileRepo<'a> {
    pool: &'a DbPool,
}

impl<'a> FileRepo<'a> {
    pub fn new(pool: &'a DbPool) -> FileRepo<'a> {
        FileRepo { pool }
    }

    pub async fn get_files(&self) -> Result<Vec<File>, RepoError> {
        let rows = sqlx::query_as!(
            File,
            r#"SELECT f.id
                    , f.platform
                    , f.build
                    , f.version
               FROM files as f;"#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn create_file(&self, new_file: NewFile) -> Result<File, RepoError> {
        let row = sqlx::query_as!(
            File,
            r#"INSERT INTO files ( id
                                 , platform
                                 , build
                                 , version )
               VALUES ( $1
                      , $2
                      , $3
                      , $4 )
               RETURNING id
                       , platform
                       , build
                       , version;"#,
            Uuid::new_v4(),
            new_file.platform,
            new_file.build,
            new_file.version
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn has_file<S1, S2, S3>(
        &self,
        platform: S1,
        build: S2,
        version: S3,
    ) -> Result<bool, RepoError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let row = sqlx::query!(
            r#"SELECT count(1) as "count!"
               FROM files as f
               WHERE f.platform = $1
                 AND f.build = $2
                 AND f.version = $3;"#,
            platform.as_ref(),
            build.as_ref(),
            version.as_ref(),
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row.count > 0)
    }
}
