use crate::models;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    #[serde(flatten)]
    pub platforms: HashMap<String, Platform>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Platform {
    #[serde(flatten)]
    pub builds: HashMap<String, Build>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Build {
    pub released: String,
    pub testing: String,
    pub link: String,
}

impl From<Vec<models::settings::Settings>> for Settings {
    fn from(v: Vec<models::settings::Settings>) -> Self {
        let platforms = v.into_iter().fold(HashMap::new(), |mut platforms, set| {
            let p = match platforms.get_mut(&set.platform) {
                None => {
                    platforms.insert(
                        set.platform.clone(),
                        Platform {
                            builds: Default::default(),
                        },
                    );
                    platforms.get_mut(&set.platform).unwrap()
                }
                Some(p) => p,
            };

            p.builds.insert(
                set.build,
                Build {
                    released: set.released_ver,
                    testing: set.testing_ver,
                    link: format!("/api/v1/download{}", set.file_path),
                },
            );

            platforms
        });

        Settings { platforms }
    }
}

impl From<Settings> for Vec<models::settings::Settings> {
    fn from(s: Settings) -> Vec<models::settings::Settings> {
        let mut res = Vec::new();

        for (pk, p) in s.platforms.iter() {
            for (bk, b) in p.builds.iter() {
                res.push(models::settings::Settings {
                    id: Default::default(),
                    platform: pk.clone(),
                    build: bk.clone(),
                    released_ver: b.released.clone(),
                    testing_ver: b.testing.clone(),
                    file_path: b.link.clone(),
                })
            }
        }

        res
    }
}
