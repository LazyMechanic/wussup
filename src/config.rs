use config as config_lib;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {}

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
