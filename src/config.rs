use config as config_lib;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub server: Server,
    #[serde(default = "default_logger")]
    pub logger: serde_yaml::Value,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Server {
    pub port: u16,
    pub env: Environment,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Prod,
    Dev,
}

fn default_logger() -> serde_yaml::Value {
    const DEFAULT_LOG4RS_SETTINGS: &str = r##"
    appenders:
      stdout:
        kind: console
        encoder:
          pattern: "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} - {h({l})} {M} {f}:{L} = {m} {n}"
    root:
      level: error
      appenders:
        - stdout
    loggers:
      wussup_lib:
        level: info
        appenders:
          - stdout
        additive: false
    "##;
    serde_yaml::from_str(DEFAULT_LOG4RS_SETTINGS).unwrap()
}

impl Config {
    #[allow(dead_code)]
    pub fn from_env() -> Result<Config, config_lib::ConfigError> {
        let mut config = config_lib::Config::new();
        config.merge(config_lib::Environment::new())?;

        let settings = config.try_into()?;
        Ok(settings)
    }

    #[allow(dead_code)]
    pub fn from_file<S: AsRef<str>>(file_path: S) -> Result<Config, config_lib::ConfigError> {
        let mut config = config_lib::Config::new();
        config.merge(config_lib::File::new(
            file_path.as_ref(),
            config_lib::FileFormat::Yaml,
        ))?;

        let settings = config.try_into()?;
        Ok(settings)
    }
}
