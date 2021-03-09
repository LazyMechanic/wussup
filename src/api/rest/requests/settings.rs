use crate::api::rest::prelude::*;

#[derive(serde::Deserialize, Debug)]
pub struct UpdateSettings {
    #[serde(flatten)]
    pub payload: api_models::settings::Settings,
}

#[derive(serde::Deserialize, Debug)]
pub struct AddPlatform {
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct AddBuild {
    pub name: String,
}
