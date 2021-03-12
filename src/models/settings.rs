use uuid::Uuid;

#[derive(Debug)]
pub struct Settings {
    pub id: Uuid,
    pub platform: String,
    pub build: String,
    pub released_file_id: Uuid,
    pub testing_file_id: Uuid,
}

#[derive(Debug)]
pub struct UpdateSettings {
    pub id: Uuid,
    pub platform: String,
    pub build: String,
    pub released_ver: String,
    pub testing_ver: String,
}

#[derive(Debug)]
pub struct Build {
    pub name: String,
}

#[derive(Debug)]
pub struct Platform {
    pub name: String,
}

pub struct NewSettings {
    pub platform: String,
    pub build: String,
    pub released_file_id: Uuid,
    pub testing_file_id: Uuid,
}

#[derive(Debug)]
pub struct NewBuild {
    pub name: String,
}

#[derive(Debug)]
pub struct NewPlatform {
    pub name: String,
}
