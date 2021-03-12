use uuid::Uuid;

#[derive(Debug)]
pub struct NewFile {
    pub platform: String,
    pub build: String,
    pub version: String,
}

#[derive(Debug)]
pub struct File {
    pub id: Uuid,
    pub platform: String,
    pub build: String,
    pub version: String,
}
