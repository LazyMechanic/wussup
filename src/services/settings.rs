use crate::models::settings::*;
use crate::services::local_prelude::*;

pub struct SettingsService {
    db: DbPool,
}

impl SettingsService {
    pub fn new(db: DbPool) -> SettingsService {
        SettingsService { db }
    }

    pub async fn get_platforms(&self) -> Result<Vec<Platform>, ServiceError> {
        let rows = self.db.settings_repo().get_platforms().await?;
        Ok(rows)
    }

    pub async fn get_builds(&self) -> Result<Vec<Build>, ServiceError> {
        let rows = self.db.settings_repo().get_builds().await?;
        Ok(rows)
    }

    pub async fn get_settings(&self) -> Result<Vec<UpdateSettings>, ServiceError> {
        let rows = self.db.settings_repo().get_settings().await?;
        Ok(rows)
    }

    pub async fn create_platform(
        &self,
        platform: NewPlatform,
    ) -> Result<Vec<Platform>, ServiceError> {
        self.db.settings_repo().create_platform(platform).await?;
        let rows = self.db.settings_repo().get_platforms().await?;
        Ok(rows)
    }

    pub async fn create_build(&self, build: NewBuild) -> Result<Vec<Build>, ServiceError> {
        self.db.settings_repo().create_build(build).await?;
        let rows = self.db.settings_repo().get_builds().await?;
        Ok(rows)
    }

    pub async fn delete_platform<S>(&self, name: S) -> Result<Vec<Platform>, ServiceError>
    where
        S: AsRef<str>,
    {
        self.db.settings_repo().delete_platform(name).await?;
        let rows = self.db.settings_repo().get_platforms().await?;
        Ok(rows)
    }

    pub async fn delete_build<S>(&self, name: S) -> Result<Vec<Build>, ServiceError>
    where
        S: AsRef<str>,
    {
        self.db.settings_repo().delete_build(name).await?;
        let rows = self.db.settings_repo().get_builds().await?;
        Ok(rows)
    }

    pub async fn update_settings(
        &self,
        settings: Vec<NewSettings>,
    ) -> Result<Vec<Settings>, ServiceError> {
        let rows = self.db.settings_repo().update_settings(settings).await?;
        Ok(rows)
    }
}
