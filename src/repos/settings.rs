use crate::models::settings::*;
use crate::repos::DbPool;
use crate::repos::Error;

use uuid::Uuid;

pub struct SettingsRepo<'a> {
    pool: &'a DbPool,
}

impl<'a> SettingsRepo<'a> {
    pub fn new(pool: &'a DbPool) -> SettingsRepo<'a> {
        SettingsRepo { pool }
    }

    pub async fn get_platforms(&self) -> Result<Vec<Platform>, Error> {
        let rows = sqlx::query_as!(
            Platform,
            r#"SELECT p.name
               FROM platforms as p;"#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_builds(&self) -> Result<Vec<Build>, Error> {
        let rows = sqlx::query_as!(
            Build,
            r#"SELECT b.name
               FROM builds as b;"#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_settings(&self) -> Result<Vec<Settings>, Error> {
        let rows = sqlx::query_as!(
            Settings,
            r#"SELECT s.id
                    , s.platform
                    , s.build
                    , s.released_ver
                    , s.testing_ver
                    , s.file_path
               FROM settings as s;"#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn add_platform(&self, p: Platform) -> Result<Platform, Error> {
        let row = sqlx::query_as!(
            Platform,
            r#"INSERT INTO platforms ( name )
               VALUES ( $1 )
               RETURNING name;"#,
            p.name,
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn add_build(&self, p: Build) -> Result<Build, Error> {
        let row = sqlx::query_as!(
            Build,
            r#"INSERT INTO builds ( name )
               VALUES ( $1 )
               RETURNING name;"#,
            p.name,
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn full_update_settings(
        &self,
        settings: Vec<Settings>,
    ) -> Result<Vec<Settings>, Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(r#"DELETE FROM settings as s;"#)
            .execute(&mut tx)
            .await?;

        let mut rows = Vec::with_capacity(settings.capacity());

        for s in settings.into_iter() {
            let row = sqlx::query_as!(
                Settings,
                r#"INSERT INTO settings ( id
                                        , platform
                                        , build
                                        , released_ver
                                        , testing_ver
                                        , file_path )
                   VALUES ( $1
                          , $2 
                          , $3 
                          , $4 
                          , $5 
                          , $6 )
                   RETURNING id
                           , platform
                           , build
                           , released_ver
                           , testing_ver
                           , file_path;"#,
                Uuid::new_v4(),
                s.platform,
                s.build,
                s.released_ver,
                s.testing_ver,
                s.file_path,
            )
            .fetch_one(&mut tx)
            .await?;

            rows.push(row);
        }

        tx.commit().await?;
        Ok(rows)
    }
}
