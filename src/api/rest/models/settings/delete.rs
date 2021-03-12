use crate::api::rest::prelude::*;

#[derive(Debug, serde::Serialize)]
pub struct DeletePlatformResponse {
    pub platforms: Vec<String>,
}

impl From<Vec<db_models::settings::Platform>> for DeletePlatformResponse {
    fn from(f: Vec<db_models::settings::Platform>) -> Self {
        Self {
            platforms: f.into_iter().map(|p| p.name).collect(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct DeleteBuildResponse {
    pub builds: Vec<String>,
}

impl From<Vec<db_models::settings::Build>> for DeleteBuildResponse {
    fn from(f: Vec<db_models::settings::Build>) -> Self {
        Self {
            builds: f.into_iter().map(|b| b.name).collect(),
        }
    }
}
