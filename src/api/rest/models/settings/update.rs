use crate::api::rest::models::settings::*;
use crate::api::rest::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct UpdateSettingsRequest {
    pub settings: Vec<NewSettings>,
}

impl From<UpdateSettingsRequest> for Vec<db_models::settings::NewSettings> {
    fn from(f: UpdateSettingsRequest) -> Self {
        f.settings
            .into_iter()
            .map(|s| db_models::settings::NewSettings {
                platform: s.platform,
                build: s.build,
                released_file_id: s.released_file_id,
                testing_file_id: s.testing_file_id,
            })
            .collect()
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateSettingsResponse {
    pub settings: Vec<Settings>,
}

impl From<Vec<db_models::settings::Settings>> for UpdateSettingsResponse {
    fn from(v: Vec<db_models::settings::Settings>) -> Self {
        Self {
            settings: v.into_iter().map(|s| s.into()).collect(),
        }
    }
}
