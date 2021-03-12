use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

use crate::config;
use crate::models::auth::NewClient;
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
        self.cfg.enable
    }

    pub async fn authorize<S1, S2>(
        &self,
        access_token_encoded: S1,
        refresh_token_encoded: S2,
    ) -> Result<(AccessTokenDecoded, RefreshTokenDecoded), ServiceError>
    where
        S1: AsRef<str>,
        S2: AsRef<[u8]>,
    {
        let access_token_decoded =
            AccessTokenDecoded::decode(&self.cfg.secret, access_token_encoded)?;

        // If access token expires
        if Utc::now().naive_utc() >= access_token_decoded.exp {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "access token expires"
            )));
        }

        let refresh_token_decoded = RefreshTokenDecoded::decode(refresh_token_encoded)?;

        Ok((access_token_decoded, refresh_token_decoded))
    }

    pub async fn login<S>(
        &self,
        fingerprint: S,
        password: ClientPassword,
    ) -> Result<(AccessTokenDecoded, RefreshTokenDecoded), ServiceError>
    where
        S: Into<String>,
    {
        // Compare a password with the master password from the config
        if password != self.cfg.password {
            return Err(ServiceError::AuthError(anyhow::anyhow!("wrong password")));
        }

        // Create new client
        let client = NewClient {
            refresh_token: Uuid::new_v4(),
            refresh_token_exp: utils::expires_timestamp(self.cfg.refresh_expires),
            client_id: Uuid::new_v4(),
            fingerprint: fingerprint.into(),
        };

        // Save new auth session
        let client = self.db.auth_repo().create_client(client).await?;

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

        Ok((access_token, refresh_token))
    }

    pub async fn refresh_tokens<S>(
        &self,
        fingerprint: S,
        jwt: Jwt,
    ) -> Result<(AccessTokenDecoded, RefreshTokenDecoded), ServiceError>
    where
        S: Into<String>,
    {
        let fingerprint = fingerprint.into();

        // Remove client
        let old_client = self
            .db
            .auth_repo()
            .delete_client(jwt.refresh_token.token)
            .await?;

        // If refresh token expires
        if Utc::now().naive_utc() >= old_client.refresh_token_exp {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "refresh token expires"
            )));
        }

        // If old fingerprint and new are not equal
        if old_client.fingerprint != fingerprint {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "fingerprints not equal"
            )));
        }

        // Create new client
        let new_client = NewClient {
            refresh_token: Uuid::new_v4(),
            refresh_token_exp: utils::expires_timestamp(self.cfg.refresh_expires),
            client_id: old_client.client_id,
            fingerprint,
        };

        // Save new auth session
        let new_client = self.db.auth_repo().create_client(new_client).await?;

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

        Ok((new_access_token, new_refresh_token))
    }

    pub async fn logout(&self, jwt: Jwt) -> Result<(), ServiceError> {
        // Get client
        let client = self
            .db
            .auth_repo()
            .get_client(jwt.refresh_token.token)
            .await?;

        // If clients not eq (from token and from db)
        if client.client_id != jwt.access_token.client_id {
            return Err(ServiceError::AuthError(anyhow::anyhow!(
                "client id in access token does not equal with client id in db"
            )));
        }

        // Delete auth session
        self.db
            .auth_repo()
            .delete_client(jwt.refresh_token.token)
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
    #[serde(with = "naive_date_time_format")]
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

    pub fn encode(&self) -> Result<AccessTokenEncoded, ServiceError> {
        let token =
            encode_token(&self.secret, self).map_err(|err| ServiceError::AuthError(err.into()))?;

        log::debug!("access token encode: {:?}", token);
        Ok(token)
    }

    pub fn decode<S1, S2>(secret: S1, token: S2) -> Result<AccessTokenDecoded, ServiceError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut token_decoded: AccessTokenDecoded = decode_token(secret.as_ref(), token)
            .map_err(|err| ServiceError::AuthError(err.into()))?;
        token_decoded.secret = secret.as_ref().to_owned();

        log::debug!("access token decode: {:?}", token_decoded);
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
    #[serde(with = "naive_date_time_format")]
    exp: NaiveDateTime,
}

impl RefreshTokenDecoded {
    pub fn new(exp: NaiveDateTime) -> RefreshTokenDecoded {
        RefreshTokenDecoded {
            token: Uuid::new_v4(),
            exp,
        }
    }

    pub fn encode(&self) -> Result<RefreshTokenEncoded, ServiceError> {
        let encoded_str =
            serde_json::to_string(self).map_err(|err| ServiceError::CommonError(err.into()))?;
        log::debug!("refresh token encode (json): {:?}", encoded_str);

        let encoded_b64 = base64::encode_config(encoded_str, base64::URL_SAFE_NO_PAD);
        log::debug!("refresh token encode (base64): {:?}", encoded_b64);

        Ok(encoded_b64)
    }

    pub fn decode<T>(b64: T) -> Result<RefreshTokenDecoded, ServiceError>
    where
        T: AsRef<[u8]>,
    {
        let decoded_b64 = base64::decode_config(b64, base64::URL_SAFE_NO_PAD)
            .map_err(|err| ServiceError::CommonError(err.into()))?;
        log::debug!("refresh token decode (base64): {:?}", decoded_b64);

        let decoded_str =
            String::from_utf8(decoded_b64).map_err(|err| ServiceError::CommonError(err.into()))?;
        log::debug!("refresh token decode (json): {:?}", decoded_str);

        let refresh_token = serde_json::from_str(&decoded_str)
            .map_err(|err| ServiceError::CommonError(err.into()))?;
        log::debug!("refresh token decode (obj): {:?}", refresh_token);

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

mod naive_date_time_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = i64::deserialize(deserializer)?;
        Ok(NaiveDateTime::from_timestamp(secs, 0))
    }
}
