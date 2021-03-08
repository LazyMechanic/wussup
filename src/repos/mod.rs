pub mod error;
pub mod prelude;
pub mod settings;

pub use crate::repos::error::Error;

use crate::config;
use crate::repos::settings::SettingsRepo;
use crate::services::error::SettingsError;

use sqlx::Acquire;
use std::sync::Arc;
use uuid::Uuid;

pub type Db = sqlx::Postgres;
pub type DbPool = sqlx::Pool<Db>;
pub type DbPoolConnection = sqlx::pool::PoolConnection<Db>;

pub async fn connect(cfg: &config::Db) -> Result<DbPool, Error> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(cfg.max_connections)
        .connect(&cfg.url)
        .await?;

    Ok(pool)
}

pub trait DbPoolExt {
    fn settings_repo(&self) -> SettingsRepo;
}

impl DbPoolExt for DbPool {
    fn settings_repo(&self) -> SettingsRepo {
        SettingsRepo::new(self)
    }
}
