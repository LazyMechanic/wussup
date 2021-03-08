use crate::repos;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error(transparent)]
    TokenEncodeError(jsonwebtoken::errors::Error),
    #[error(transparent)]
    TokenDecodeError(jsonwebtoken::errors::Error),
    #[error("authorization error: {0}")]
    AuthorizationError(String),
    #[error("login error: {0}")]
    LoginError(String),
    #[error("refresh tokens error: {0}")]
    RefreshTokensError(String),
    #[error("logout error: {0}")]
    LogoutError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum SettingsError {
    #[error(transparent)]
    RepoError(#[from] repos::Error),
}
