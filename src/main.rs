use wussup_lib::config::Config;

const DEFAULT_CONFIG_PATH: &str = "config.yaml";

fn main() -> anyhow::Result<()> {
    let cfg = Config::from_file(DEFAULT_CONFIG_PATH)?;
    println!("config: {:#?}", cfg);

    Ok(())
}
