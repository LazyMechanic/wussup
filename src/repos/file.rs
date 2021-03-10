use crate::models::file::*;
use crate::repos::DbPool;
use crate::repos::RepoError;

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
                    , f.path
               FROM files as f;"#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn add_file(&self, file: File) -> Result<File, RepoError> {
        let rows = sqlx::query_as!(
            File,
            r#"INSERT INTO files ( id
                                 , path )
               VALUES ( $1
                      , $2 )
               RETURNING id
                       , path;"#,
            file.id,
            file.path,
        )
        .fetch_one(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn has_file<S: AsRef<str>>(&self, path: S) -> Result<bool, RepoError> {
        let row = sqlx::query!(
            r#"SELECT count(1) as "count!"
               FROM files as f
               WHERE f.path = $1;"#,
            path.as_ref(),
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row.count > 0)
    }
}
