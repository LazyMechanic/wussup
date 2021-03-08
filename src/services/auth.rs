use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::config;
use crate::models;
use crate::repos::prelude::*;
use crate::services::utils;

use super::local_prelude::*;

const ACCESS_TOKEN_PREFIX: &str = "Bearer ";
pub const REFRESH_TOKEN_COOKIE_NAME: &str = "refreshToken";

pub struct AuthService {
    cfg: config::Auth,
    db: DbPool,
}

impl AuthService {
    pub fn new(cfg: config::Auth, db: DbPool) -> AuthService {
        AuthService { cfg, db }
    }

    pub fn is_enable(&self) -> bool {
        return self.cfg.enable;
    }

    pub async fn authorize<S1, S2>(
        &self,
        access_token_encoded: S1,
        refresh_token_encoded: S2,
    ) -> Result<(AccessTokenDecoded, RefreshTokenDecoded), AuthError>
    where
        S1: AsRef<str>,
        S2: AsRef<[u8]>,
    {
        let access_token_decoded =
            AccessTokenDecoded::decode(&self.cfg.secret, access_token_encoded)?;

        // If access token expires
        if Utc::now().naive_utc() >= access_token_decoded.exp {
            return Err(AuthError::AuthorizationError(
                "access token expires".to_string(),
            ));
        }

        let refresh_token_decoded = RefreshTokenDecoded::decode(refresh_token_encoded)?;

        Ok((access_token_decoded, refresh_token_decoded))
    }

    pub async fn login<S>(
        &self,
        fingerprint: S,
        password: ClientPassword,
    ) -> Result<(AccessTokenDecoded, RefreshTokenDecoded), AuthError>
    where
        S: Into<String>,
    {
        // Compare a password with the master password from the config
        if password != self.cfg.password {
            return Err(AuthError::LoginError("wrong password".to_owned()));
        }

        // Create new client
        let client = models::auth::Client::new(
            Uuid::new_v4(),
            utils::expires_timestamp(self.cfg.refresh_expires),
            fingerprint,
            Uuid::new_v4(),
        );

        // Create access token
        let access_token = AccessTokenDecoded::new(
            client.client_id,
            self.cfg.secret.clone(),
            utils::expires_timestamp(self.cfg.access_expires),
        );

        // Create refresh token
        let refresh_token = RefreshTokenDecoded {
            token: client.refresh_token,
            exp: client.refresh_token_exp,
        };

        // Save new auth session
        self.db.auth_repo().add_client(client).await?;

        Ok((access_token, refresh_token))
    }

    pub async fn refresh_tokens<S>(
        &self,
        fingerprint: S,
        jwt: Jwt,
    ) -> Result<(AccessTokenDecoded, RefreshTokenDecoded), AuthError>
    where
        S: Into<String>,
    {
        let fingerprint = fingerprint.into();

        // Remove client
        let old_client = self
            .db
            .auth_repo()
            .remove_client(jwt.refresh_token.token)
            .await?;

        // If refresh token expires
        if Utc::now().naive_utc() >= old_client.refresh_token_exp {
            return Err(AuthError::RefreshTokensError(
                "refresh token expires".to_string(),
            ));
        }

        // If old fingerprint and new are not equal
        if old_client.fingerprint != fingerprint {
            return Err(AuthError::RefreshTokensError(
                "fingerprints not equal".to_string(),
            ));
        }

        // Create new client
        let new_client = models::auth::Client::new(
            Uuid::new_v4(),
            utils::expires_timestamp(self.cfg.refresh_expires),
            fingerprint,
            old_client.client_id,
        );

        // Create new access token
        let new_access_token = AccessTokenDecoded::new(
            new_client.client_id,
            self.cfg.secret.clone(),
            utils::expires_timestamp(self.cfg.access_expires),
        );

        // Create new refresh token
        let new_refresh_token = RefreshTokenDecoded {
            token: new_client.refresh_token,
            exp: new_client.refresh_token_exp,
        };

        // Save new auth session
        self.db.auth_repo().add_client(new_client).await?;

        Ok((new_access_token, new_refresh_token))
    }

    pub async fn logout(&self, jwt: Jwt) -> Result<(), AuthError> {
        // Get client
        let client = self
            .db
            .auth_repo()
            .get_client(jwt.refresh_token.token)
            .await?;

        // If clients not eq (from token and from db)
        if client.client_id != jwt.access_token.client_id {
            return Err(AuthError::LogoutError(
                "client id in access token does not equal with client id in db".to_string(),
            ));
        }

        // Delete auth session
        self.db
            .auth_repo()
            .remove_client(jwt.refresh_token.token)
            .await?;

        Ok(())
    }
}

pub type ClientId = Uuid;
pub type ClientPassword = String;

pub type AccessTokenEncoded = String;
pub type RefreshTokenEncoded = String;
pub type RefreshToken = Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessTokenDecoded {
    // seconds since the epoch
    exp: NaiveDateTime,
    client_id: ClientId,
    #[serde(skip)]
    secret: String,
}

impl AccessTokenDecoded {
    fn new<S>(client_id: ClientId, secret: S, exp: NaiveDateTime) -> AccessTokenDecoded
    where
        S: Into<String>,
    {
        AccessTokenDecoded {
            exp,
            client_id,
            secret: secret.into(),
        }
    }

    pub fn encode(&self) -> Result<AccessTokenEncoded, AuthError> {
        let token = encode_token(&self.secret, self).map_err(AuthError::TokenEncodeError)?;
        Ok(token)
    }

    pub fn decode<S1, S2>(secret: S1, token: S2) -> Result<AccessTokenDecoded, AuthError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut token_decoded: AccessTokenDecoded =
            decode_token(secret.as_ref(), token).map_err(AuthError::TokenDecodeError)?;
        token_decoded.secret = secret.as_ref().to_owned();
        Ok(token_decoded)
    }

    pub fn exp(&self) -> NaiveDateTime {
        self.exp
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }
}

impl Default for AccessTokenDecoded {
    fn default() -> Self {
        Self {
            exp: Utc::now().naive_utc(),
            client_id: Default::default(),
            secret: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct RefreshTokenDecoded {
    token: RefreshToken,
    exp: NaiveDateTime,
}

impl RefreshTokenDecoded {
    pub fn new(exp: NaiveDateTime) -> RefreshTokenDecoded {
        RefreshTokenDecoded {
            token: Uuid::new_v4(),
            exp,
        }
    }

    pub fn encode(&self) -> Result<RefreshTokenEncoded, AuthError> {
        let encoded_str = serde_json::to_string(self)?;
        let encoded_b64 = base64::encode(encoded_str);
        Ok(encoded_b64)
    }

    pub fn decode<T>(b64: T) -> Result<RefreshTokenDecoded, AuthError>
    where
        T: AsRef<[u8]>,
    {
        let decoded_b64 = base64::decode(b64)?;
        let decoded_str = String::from_utf8(decoded_b64)?;
        let refresh_token = serde_json::from_str(&decoded_str)?;
        Ok(refresh_token)
    }

    pub fn token(&self) -> RefreshToken {
        self.token
    }

    pub fn exp(&self) -> NaiveDateTime {
        self.exp
    }
}

impl Default for RefreshTokenDecoded {
    fn default() -> Self {
        Self {
            token: Default::default(),
            exp: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Jwt {
    pub access_token: AccessTokenDecoded,
    pub refresh_token: RefreshTokenDecoded,
}

fn encode_token<S, T>(secret: S, token_decoded: &T) -> Result<String, jsonwebtoken::errors::Error>
where
    S: AsRef<str>,
    T: Serialize,
{
    let token = jsonwebtoken::encode(
        &Header::default(),
        token_decoded,
        &EncodingKey::from_secret(secret.as_ref().as_bytes()),
    )?;

    Ok(token)
}

fn decode_token<T, S1, S2>(secret: S1, token: S2) -> Result<T, jsonwebtoken::errors::Error>
where
    T: DeserializeOwned,
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let access_token_decoded = jsonwebtoken::decode::<T>(
        token.as_ref().trim_start_matches(ACCESS_TOKEN_PREFIX),
        &DecodingKey::from_secret(secret.as_ref().as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)?;

    Ok(access_token_decoded)
}
