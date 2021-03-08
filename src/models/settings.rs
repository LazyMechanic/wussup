use uuid::Uuid;

#[derive(Debug)]
pub struct Settings {
    pub platform: String,
    pub build: String,
    pub released_ver: String,
    pub testing_ver: String,
    pub file_path: String,
}

#[derive(Debug)]
pub struct Build {
    pub name: String,
}

#[derive(Debug)]
pub struct Platform {
    pub name: String,
}
