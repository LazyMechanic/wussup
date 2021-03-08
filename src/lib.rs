pub mod config;

mod api;
mod models;
mod repos;
mod services;

use crate::api::context::Context;
use crate::config::Config;
use crate::services::prelude;

use crate::services::auth::AuthService;
use crate::services::settings::SettingsService;
use futures::prelude::*;
use std::sync::Arc;

pub async fn run(cfg: Config) -> anyhow::Result<()> {
    let db = repos::connect(&cfg.db).await?;
    let ctx = Context {
        auth_service: Arc::new((AuthService::new(cfg.auth.clone()))),
        settings_service: Arc::new(SettingsService::new(db)),
    };

    tokio::spawn(api::rest::run(ctx, cfg));

    future::pending().await
}
