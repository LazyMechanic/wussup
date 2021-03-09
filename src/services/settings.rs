use crate::models::settings::*;
use crate::repos::prelude::*;
use crate::services::local_prelude::*;

pub struct SettingsService {
    db: DbPool,
}

impl SettingsService {
    pub fn new(db: DbPool) -> SettingsService {
        SettingsService { db }
    }

    pub async fn get_platforms(&self) -> Result<Vec<Platform>, SettingsError> {
        let rows = self.db.settings_repo().get_platforms().await?;
        Ok(rows)
    }

    pub async fn get_builds(&self) -> Result<Vec<Build>, SettingsError> {
        let rows = self.db.settings_repo().get_builds().await?;
        Ok(rows)
    }

    pub async fn get_settings(&self) -> Result<Vec<Settings>, SettingsError> {
        let rows = self.db.settings_repo().get_settings().await?;
        Ok(rows)
    }

    pub async fn add_platform(&self, platform: Platform) -> Result<Vec<Platform>, SettingsError> {
        self.db.settings_repo().add_platform(platform).await?;
        let rows = self.db.settings_repo().get_platforms().await?;
        Ok(rows)
    }

    pub async fn add_build(&self, build: Build) -> Result<Vec<Build>, SettingsError> {
        self.db.settings_repo().add_build(build).await?;
        let rows = self.db.settings_repo().get_builds().await?;
        Ok(rows)
    }

    pub async fn delete_platform(&self, name: String) -> Result<Vec<Platform>, SettingsError> {
        self.db.settings_repo().delete_platform(name).await?;
        let rows = self.db.settings_repo().get_platforms().await?;
        Ok(rows)
    }

    pub async fn delete_build(&self, name: String) -> Result<Vec<Build>, SettingsError> {
        self.db.settings_repo().delete_build(name).await?;
        let rows = self.db.settings_repo().get_builds().await?;
        Ok(rows)
    }

    pub async fn update_settings(
        &self,
        settings: Vec<Settings>,
    ) -> Result<Vec<Settings>, SettingsError> {
        let rows = self
            .db
            .settings_repo()
            .full_update_settings(settings)
            .await?;
        Ok(rows)
    }
}
