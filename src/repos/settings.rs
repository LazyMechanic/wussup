use crate::models::settings::*;
use crate::repos::DbPool;
use crate::repos::Error;

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
            r#"SELECT s.platform
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

    pub async fn remove_settings(&self) -> Result<(), Error> {
        sqlx::query_as!(Settings, r#"DELETE FROM settings as s;"#)
            .execute(self.pool)
            .await?;

        Ok(())
    }

    pub async fn full_update_settings(&self, settings: Vec<Settings>) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        if let Err(err) = sqlx::query!(r#"DELETE FROM settings as s;"#)
            .execute(&mut tx)
            .await
        {
            tx.rollback().await?;
            return Err(Error::from(err));
        }

        for s in settings.into_iter() {
            if let Err(err) = sqlx::query!(
                r#"INSERT INTO settings ( platform
                                        , build
                                        , released_ver
                                        , testing_ver
                                        , file_path )
                   VALUES ( $1
                          , $2 
                          , $3 
                          , $4 
                          , $5 );"#,
                s.platform,
                s.build,
                s.released_ver,
                s.testing_ver,
                s.file_path,
            )
            .execute(&mut tx)
            .await
            {
                tx.rollback().await?;
                return Err(Error::from(err));
            }
        }
        tx.commit().await?;
        Ok(())
    }
}
