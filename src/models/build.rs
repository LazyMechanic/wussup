use uuid::Uuid;

#[derive(Debug)]
pub struct Build {
    pub id: Uuid,
    pub name: String,
}
