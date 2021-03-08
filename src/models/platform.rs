use uuid::Uuid;

#[derive(Debug)]
pub struct Platform {
    pub id: Uuid,
    pub name: String,
}
