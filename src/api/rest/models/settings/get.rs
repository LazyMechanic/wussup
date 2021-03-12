use crate::api::rest::models::settings::*;
use crate::api::rest::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
pub struct GetSettingsResponse {
    #[serde(flatten)]
    pub matrix: HashMap<String, PlatformMatrix>,
}

impl From<Vec<db_models::settings::UpdateSettings>> for GetSettingsResponse {
    fn from(v: Vec<db_models::settings::UpdateSettings>) -> Self {
        let mut platforms = HashMap::with_capacity(v.len());

        for s in v.into_iter() {
            let mut p = match platforms.get_mut(&s.platform) {
                Some(p) => p,
                None => {
                    platforms.insert(
                        s.platform.clone(),
                        PlatformMatrix {
                            builds: Default::default(),
                        },
                    );

                    platforms.get_mut(&s.platform).unwrap()
                }
            };

            p.builds.insert(
                s.build.clone(),
                BuildMatrix {
                    released: s.released_ver,
                    testing: s.testing_ver,
                    link: format_link(s.platform, s.build),
                },
            );
        }

        Self { matrix: platforms }
    }
}

fn format_link<S1, S2>(platform: S1, build: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    format!(
        "/api/v1/files/{}/{}/{{version}}",
        platform.as_ref(),
        build.as_ref()
    )
}

#[derive(Debug, serde::Serialize)]
pub struct GetPlatformsResponse {
    pub platforms: Vec<String>,
}

impl From<Vec<db_models::settings::Platform>> for GetPlatformsResponse {
    fn from(f: Vec<db_models::settings::Platform>) -> Self {
        Self {
            platforms: f.into_iter().map(|p| p.name).collect(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct GetBuildsResponse {
    pub builds: Vec<String>,
}

impl From<Vec<db_models::settings::Build>> for GetBuildsResponse {
    fn from(f: Vec<db_models::settings::Build>) -> Self {
        Self {
            builds: f.into_iter().map(|b| b.name).collect(),
        }
    }
}
