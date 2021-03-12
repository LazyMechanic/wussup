use std::sync::Arc;

use futures::prelude::*;

use crate::api::context::Context;
use crate::config::Config;
use crate::services::auth::AuthService;
use crate::services::file::FileService;
use crate::services::settings::SettingsService;

pub mod config;

mod api;
mod models;
mod repos;
mod services;

pub async fn run(cfg: Config) -> anyhow::Result<()> {
    let db = repos::connect(&cfg.db).await?;
    let ctx = Context {
        auth_service: Arc::new(AuthService::new(cfg.auth.clone(), db.clone())),
        settings_service: Arc::new(SettingsService::new(db.clone())),
        file_service: Arc::new(FileService::new(cfg.file.clone(), db)),
    };

    tokio::spawn(api::rest::run(ctx, cfg));

    future::pending().await
}
