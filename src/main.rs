use wussup_lib::config::Config;

const DEFAULT_CONFIG_PATH: &str = "config.yaml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_file(DEFAULT_CONFIG_PATH)?;
    init_logger(&cfg.logger)?;

    wussup_lib::run(cfg).await
}

fn init_logger(config: &serde_yaml::Value) -> anyhow::Result<()> {
    let config = serde_yaml::from_value(config.clone())?;
    log4rs::config::init_raw_config(config)?;
    Ok(())
}
