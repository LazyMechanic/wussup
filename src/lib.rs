pub mod config;

mod api;
mod services;

use crate::api::context::Context;
use crate::config::Config;
use crate::services::prelude;

use crate::services::auth::AuthService;
use futures::prelude::*;
use std::sync::Arc;

pub async fn run(cfg: Config) -> anyhow::Result<()> {
    let ctx = Context {
        auth_service: Arc::new((AuthService::new(cfg.auth.clone()))),
    };

    tokio::spawn(api::rest::run(ctx, cfg));

    future::pending().await
}
