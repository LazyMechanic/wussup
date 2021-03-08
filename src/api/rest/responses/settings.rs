use crate::api::rest::models::settings::Settings;

#[derive(Debug, serde::Serialize)]
pub struct GetSettings {
    #[serde(flatten)]
    pub payload: Settings,
}

impl GetSettings {
    pub fn new(s: Settings) -> GetSettings {
        GetSettings { payload: s }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct GetPlatforms {
    pub platforms: Vec<String>,
}

impl GetPlatforms {
    pub fn new(p: Vec<String>) -> GetPlatforms {
        GetPlatforms { platforms: p }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct GetBuilds {
    pub builds: Vec<String>,
}

impl GetBuilds {
    pub fn new(b: Vec<String>) -> GetBuilds {
        GetBuilds { builds: b }
    }
}
