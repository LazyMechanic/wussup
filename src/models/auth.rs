use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct Client {
    pub refresh_token: Uuid,
    pub refresh_token_exp: NaiveDateTime,
    pub client_id: Uuid,
    pub fingerprint: String,
}

impl Client {
    pub fn new<S: Into<String>>(
        refresh_token: Uuid,
        refresh_token_exp: NaiveDateTime,
        fingerprint: S,
        client_id: Uuid,
    ) -> Client {
        Client {
            refresh_token,
            refresh_token_exp,
            fingerprint: fingerprint.into(),
            client_id,
        }
    }
}
