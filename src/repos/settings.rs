use crate::models::build::Build;
use crate::models::platform::*;
use crate::models::settings::Settings;
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
            r#"SELECT p.id
                    , p.name
               FROM platforms as p;"#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_builds(&self) -> Result<Vec<Build>, Error> {
        let rows = sqlx::query_as!(
            Build,
            r#"SELECT b.id
                    , b.name
               FROM builds as b;"#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_settings(&self) -> Result<Vec<Settings>, Error> {
        let rows = sqlx::query_as!(
            Settings,
            r#"SELECT p.name         as "platform!"
                    , b.name         as "build!"
                    , s.released_ver as "released_ver!"
                    , s.testing_ver  as "testing_ver!"
                    , s.file_path    as "file_path!"
               FROM settings as s
               LEFT JOIN platforms as p ON s.platform_id = p.id
               LEFT JOIN builds as b ON s.build_id = b.id;"#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }
}
