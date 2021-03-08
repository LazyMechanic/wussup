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

impl From<Vec<models::Settings>> for Settings {
    fn from(v: Vec<models::Settings>) -> Self {
        let platforms = v.into_iter().fold(HashMap::new(), |mut platforms, set| {
            let mut p = match platforms.get_mut(&set.platform) {
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
