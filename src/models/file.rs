use uuid::Uuid;

#[derive(Debug)]
pub struct File {
    pub id: Uuid,
    pub path: String,
}
