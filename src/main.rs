mod cli;

use crate::cli::Cli;
use wussup_lib::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_args();
    let cfg = Config::from_file(cli.config)?;
    init_logger(&cfg.logger)?;

    wussup_lib::run(cfg).await
}

fn init_logger(config: &serde_yaml::Value) -> anyhow::Result<()> {
    let config = serde_yaml::from_value(config.clone())?;
    log4rs::config::init_raw_config(config)?;
    Ok(())
}
