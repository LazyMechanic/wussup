use crate::repos;
use std::string::FromUtf8Error;

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
    #[error(transparent)]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    CookieParseError(#[from] cookie::ParseError),
    #[error(transparent)]
    RepoError(#[from] repos::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum SettingsError {
    #[error(transparent)]
    RepoError(#[from] repos::Error),
}
