use crate::models::auth::*;
use crate::repos::DbPool;
use crate::repos::RepoError;
use uuid::Uuid;

pub struct AuthRepo<'a> {
    pool: &'a DbPool,
}

impl<'a> AuthRepo<'a> {
    pub fn new(pool: &'a DbPool) -> AuthRepo<'a> {
        AuthRepo { pool }
    }

    pub async fn create_client(&self, new_client: NewClient) -> Result<Client, RepoError> {
        let row = sqlx::query_as!(
            Client,
            r#"INSERT INTO sessions ( refresh_token
                                    , refresh_token_exp
                                    , fingerprint
                                    , client_id )
               VALUES ( $1
                      , $2 
                      , $3 
                      , $4 )
               RETURNING refresh_token
                       , refresh_token_exp
                       , fingerprint
                       , client_id;"#,
            new_client.refresh_token,
            new_client.refresh_token_exp,
            new_client.fingerprint,
            new_client.client_id,
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn delete_client(&self, refresh_token: Uuid) -> Result<Client, RepoError> {
        let row = sqlx::query_as!(
            Client,
            r#"DELETE FROM sessions as s
               WHERE s.refresh_token = $1
               RETURNING s.refresh_token
                       , s.refresh_token_exp
                       , s.fingerprint
                       , s.client_id;"#,
            refresh_token,
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_client(&self, refresh_token: Uuid) -> Result<Client, RepoError> {
        let row = sqlx::query_as!(
            Client,
            r#"SELECT s.refresh_token
                    , s.refresh_token_exp
                    , s.fingerprint
                    , s.client_id
               FROM sessions as s
               WHERE s.refresh_token = $1;"#,
            refresh_token,
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }
}
