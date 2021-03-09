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

    pub async fn add_client(&self, client: Client) -> Result<(), RepoError> {
        sqlx::query_as!(
            Client,
            r#"INSERT INTO sessions ( refresh_token
                                  , refresh_token_exp
                                  , fingerprint
                                  , client_id )
               VALUES ( $1
                      , $2 
                      , $3 
                      , $4 );"#,
            client.refresh_token,
            client.refresh_token_exp,
            client.fingerprint,
            client.client_id,
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn remove_client(&self, refresh_token: Uuid) -> Result<Client, RepoError> {
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
