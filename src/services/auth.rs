use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::config;
use crate::services::utils;

use super::local_prelude::*;

const ACCESS_TOKEN_PREFIX: &str = "Bearer ";
pub const REFRESH_TOKEN_COOKIE_NAME: &str = "refreshToken";

pub struct AuthService {
    cfg: config::Auth,
    clients: RwLock<HashMap<RefreshToken, Client>>,
}

impl AuthService {
    pub fn new(cfg: config::Auth) -> AuthService {
        AuthService {
            cfg,
            clients: Default::default(),
        }
    }

    pub async fn authorize<S>(&self, token: S) -> Result<Claims, AuthError>
    where
        S: AsRef<str>,
    {
        // Decode token to claims
        let claims: Claims =
            decode_token(&self.cfg.secret, token).map_err(AuthError::TokenDecodeError)?;

        // If access token expires
        if Utc::now().timestamp() >= claims.exp {
            return Err(AuthError::AuthorizationError(
                "access token expires".to_string(),
            ));
        }

        Ok(claims)
    }

    pub async fn login<S>(
        &self,
        fingerprint: S,
        password: ClientPassword,
    ) -> Result<(AccessToken, RefreshTokenEntry), AuthError>
    where
        S: Into<String>,
    {
        // Compare a password with the master password from the config
        if password != self.cfg.password {
            return Err(AuthError::LoginError("wrong password".to_owned()));
        }

        // Create new client
        let client = Client::new(
            fingerprint,
            utils::expires_timestamp(self.cfg.refresh_expires),
        );

        // Create access token
        let access_token = encode_token(
            &self.cfg.secret,
            &Claims::new(utils::expires_timestamp(self.cfg.access_expires), client.id),
        )
        .map_err(AuthError::TokenEncodeError)?;

        let refresh_token_entry = client.refresh_token_entry;

        // Save new auth session
        self.clients
            .write()
            .await
            .insert(refresh_token_entry.token, client);

        Ok((access_token, refresh_token_entry))
    }

    pub async fn refresh_tokens<S>(
        &self,
        fingerprint: S,
        jwt: Jwt,
    ) -> Result<(AccessToken, RefreshTokenEntry), AuthError>
    where
        S: Into<String>,
    {
        let fingerprint = fingerprint.into();

        // Remove client
        let old_client = self
            .clients
            .write()
            .await
            .remove(&jwt.refresh_token)
            .ok_or_else(|| {
                AuthError::RefreshTokensError(format!(
                    "client not found, refresh_token={}",
                    jwt.refresh_token
                ))
            })?;

        // If refresh token expires
        if Utc::now().timestamp() >= old_client.refresh_token_entry.exp {
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
        let new_client = Client {
            id: old_client.id,
            fingerprint: old_client.fingerprint,
            refresh_token_entry: RefreshTokenEntry::new(utils::expires_timestamp(
                self.cfg.refresh_expires,
            )),
        };

        // Create new access token
        let new_access_token = encode_token(
            &self.cfg.secret,
            &Claims::new(
                utils::expires_timestamp(self.cfg.access_expires),
                new_client.id,
            ),
        )
        .map_err(AuthError::TokenEncodeError)?;

        let new_refresh_token = new_client.refresh_token_entry;

        // Save new auth session
        self.clients
            .write()
            .await
            .insert(new_refresh_token.token, new_client);

        Ok((new_access_token, new_refresh_token))
    }

    pub async fn logout(&self, jwt: Jwt) -> Result<(), AuthError> {
        let mut clients = self.clients.write().await;

        // Get client
        let client = clients.get(&jwt.refresh_token).ok_or_else(|| {
            AuthError::LogoutError(format!(
                "client not found, refresh_token={}",
                jwt.refresh_token
            ))
        })?;

        // If clients not eq (from token and from db)
        if client.id != jwt.claims.client_id {
            return Err(AuthError::LogoutError(
                "client id in access token does not equal with client id in db".to_string(),
            ));
        }

        // Delete auth session
        clients.remove(&jwt.refresh_token);

        Ok(())
    }
}

pub type ClientId = Uuid;
pub type ClientPassword = String;
pub type AccessToken = String;
pub type RefreshToken = Uuid;
pub type WebSocketTicket = String;

#[derive(Debug, Clone)]
struct Client {
    pub id: ClientId,
    pub fingerprint: String,
    pub refresh_token_entry: RefreshTokenEntry,
}

impl Client {
    pub fn new<S: Into<String>>(fingerprint: S, refresh_token_expires: i64) -> Client {
        let id = Uuid::new_v4();
        let fingerprint = fingerprint.into();
        let refresh_token_entry = RefreshTokenEntry::new(refresh_token_expires);

        Client {
            id,
            fingerprint,
            refresh_token_entry,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RefreshTokenEntry {
    pub token: RefreshToken,
    pub exp: i64,
}

impl RefreshTokenEntry {
    pub fn new(exp: i64) -> RefreshTokenEntry {
        RefreshTokenEntry {
            token: Uuid::new_v4(),
            exp,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Claims {
    // seconds since the epoch
    exp: i64,
    client_id: ClientId,
}

impl Claims {
    fn new(exp: i64, client_id: ClientId) -> Self {
        Self { exp, client_id }
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct WebSocketTicketEntry {
    // seconds since the epoch
    pub exp: i64,
    pub client_id: ClientId,
}

impl WebSocketTicketEntry {
    fn new(exp: i64, client_id: ClientId) -> Self {
        Self { exp, client_id }
    }

    pub fn client_id(&self) -> ClientId {
        self.client_id
    }
}

#[derive(Debug, Clone)]
pub struct Jwt {
    pub claims: Claims,
    pub refresh_token: RefreshToken,
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
    let claims = jsonwebtoken::decode::<T>(
        token.as_ref().trim_start_matches(ACCESS_TOKEN_PREFIX),
        &DecodingKey::from_secret(secret.as_ref().as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)?;

    Ok(claims)
}
