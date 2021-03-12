use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct NewClient {
    pub refresh_token: Uuid,
    pub refresh_token_exp: NaiveDateTime,
    pub client_id: Uuid,
    pub fingerprint: String,
}

#[derive(Debug)]
pub struct Client {
    pub refresh_token: Uuid,
    pub refresh_token_exp: NaiveDateTime,
    pub client_id: Uuid,
    pub fingerprint: String,
}
