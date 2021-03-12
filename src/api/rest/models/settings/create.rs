use crate::api::rest::models::settings::*;
use crate::api::rest::prelude::*;

#[derive(serde::Deserialize, Debug)]
pub struct CreatePlatformRequest {
    pub name: String,
}

impl From<CreatePlatformRequest> for db_models::settings::NewPlatform {
    fn from(f: CreatePlatformRequest) -> Self {
        Self { name: f.name }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct CreatePlatformResponse {
    pub platforms: Vec<String>,
}

impl From<Vec<db_models::settings::Platform>> for CreatePlatformResponse {
    fn from(f: Vec<db_models::settings::Platform>) -> Self {
        Self {
            platforms: f.into_iter().map(|p| p.name).collect(),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateBuildRequest {
    pub name: String,
}

impl From<CreateBuildRequest> for db_models::settings::NewBuild {
    fn from(f: CreateBuildRequest) -> Self {
        Self { name: f.name }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct CreateBuildResponse {
    pub builds: Vec<String>,
}

impl From<Vec<db_models::settings::Build>> for CreateBuildResponse {
    fn from(f: Vec<db_models::settings::Build>) -> Self {
        Self {
            builds: f.into_iter().map(|b| b.name).collect(),
        }
    }
}
