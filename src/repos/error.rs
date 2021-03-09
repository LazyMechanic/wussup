#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
