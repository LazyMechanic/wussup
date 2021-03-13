use crate::models::settings::*;
use crate::repos::DbPool;
use crate::repos::RepoError;

use uuid::Uuid;

pub struct SettingsRepo<'a> {
    pool: &'a DbPool,
}

impl<'a> SettingsRepo<'a> {
    pub fn new(pool: &'a DbPool) -> SettingsRepo<'a> {
        SettingsRepo { pool }
    }

    pub async fn get_platforms(&self) -> Result<Vec<Platform>, RepoError> {
        let rows = sqlx::query_as!(
            Platform,
            r#"SELECT p.name
               FROM platforms as p;"#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_builds(&self) -> Result<Vec<Build>, RepoError> {
        let rows = sqlx::query_as!(
            Build,
            r#"SELECT b.name
               FROM builds as b;"#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_settings(&self) -> Result<Vec<UpdateSettings>, RepoError> {
        let rows = sqlx::query_as!(
            UpdateSettings,
            r#"SELECT s.id       as "id!"
                    , s.platform as "platform!"
                    , s.build    as "build!"
                    , fr.version as "released_ver!"
                    , ft.version as "testing_ver!"
               FROM settings as s
               LEFT JOIN files as fr
                      ON fr.id = s.released_file_id
               LEFT JOIN files as ft
                      ON ft.id = s.testing_file_id;"#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn create_platform(&self, new_platform: NewPlatform) -> Result<Platform, RepoError> {
        let row = sqlx::query_as!(
            Platform,
            r#"INSERT INTO platforms ( name )
               VALUES ( $1 )
               RETURNING name;"#,
            new_platform.name,
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn create_build(&self, new_build: NewBuild) -> Result<Build, RepoError> {
        let row = sqlx::query_as!(
            Build,
            r#"INSERT INTO builds ( name )
               VALUES ( $1 )
               RETURNING name;"#,
            new_build.name,
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn delete_platform<S>(&self, name: S) -> Result<Platform, RepoError>
    where
        S: AsRef<str>,
    {
        let row = sqlx::query_as!(
            Platform,
            r#"DELETE FROM platforms as p
               WHERE p.name = $1
               RETURNING name;"#,
            name.as_ref(),
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn delete_build<S>(&self, name: S) -> Result<Build, RepoError>
    where
        S: AsRef<str>,
    {
        let row = sqlx::query_as!(
            Build,
            r#"DELETE FROM builds as b
               WHERE b.name = $1
               RETURNING name;"#,
            name.as_ref(),
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn update_settings(
        &self,
        settings: Vec<NewSettings>,
    ) -> Result<Vec<Settings>, RepoError> {
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
                                        , released_file_id
                                        , testing_file_id )
                   VALUES ( $1
                          , $2
                          , $3
                          , $4
                          , $5 )
                   RETURNING id
                           , platform
                           , build
                           , released_file_id
                           , testing_file_id;"#,
                Uuid::new_v4(),
                s.platform,
                s.build,
                s.released_file_id,
                s.testing_file_id,
            )
            .fetch_one(&mut tx)
            .await?;

            rows.push(row);
        }

        tx.commit().await?;

        Ok(rows)
    }

    pub async fn has_file(&self, file_id: Uuid) -> Result<bool, RepoError> {
        let row = sqlx::query!(
            r#"SELECT count(1) as "count!"
               FROM settings as s
               WHERE s.released_file_id = $1
                  OR s.testing_file_id = $1;"#,
            file_id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row.count > 0)
    }
}
