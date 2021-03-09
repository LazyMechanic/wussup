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
}
