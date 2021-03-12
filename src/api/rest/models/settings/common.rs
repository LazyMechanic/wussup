use crate::api::rest::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

/*

{
  "settings": [
    {
      "platform": "win",
      "build": "stable",
      "testingFileId": "<UUID>",
      "releasedFileId": "<UUID>",
    },
    {
      "platform": "linux",
      "build": "stable",
      "testingFileId": "<UUID>",
      "releasedFileId": "<UUID>",
    },
    {
      "platform": "mac",
      "build": "stable",
      "testingFileId": "<UUID>",
      "releasedFileId": "<UUID>",
    },
    {
      "platform": "osx",
      "build": "stable",
      "testingFileId": "<UUID>",
      "releasedFileId": "<UUID>",
    }
  ]
}

*/

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSettings {
    pub platform: String,
    pub build: String,
    pub released_file_id: Uuid,
    pub testing_file_id: Uuid,
}

impl From<NewSettings> for db_models::settings::NewSettings {
    fn from(f: NewSettings) -> Self {
        Self {
            platform: f.platform,
            build: f.build,
            released_file_id: f.released_file_id,
            testing_file_id: f.testing_file_id,
        }
    }
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub id: Uuid,
    pub platform: String,
    pub build: String,
    pub released_file_id: Uuid,
    pub testing_file_id: Uuid,
}

impl From<Settings> for db_models::settings::Settings {
    fn from(f: Settings) -> Self {
        Self {
            id: f.id,
            platform: f.platform,
            build: f.build,
            released_file_id: f.released_file_id,
            testing_file_id: f.testing_file_id,
        }
    }
}

impl From<db_models::settings::Settings> for Settings {
    fn from(f: db_models::settings::Settings) -> Self {
        Self {
            id: f.id,
            platform: f.platform,
            build: f.build,
            released_file_id: f.released_file_id,
            testing_file_id: f.testing_file_id,
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformMatrix {
    #[serde(flatten)]
    pub builds: HashMap<String, BuildMatrix>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildMatrix {
    pub released: String,
    pub testing: String,
    pub link: String,
}
