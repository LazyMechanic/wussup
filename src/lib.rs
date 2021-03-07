pub mod config;

mod api;

use crate::api::context::Context;
use crate::config::Config;

use futures::prelude::*;

pub async fn run(cfg: Config) -> anyhow::Result<()> {
    let ctx = Context {};
    tokio::spawn(api::rest::run(ctx, cfg));

    future::pending().await
}
