#[derive(serde::Deserialize, Debug)]
pub struct Login {
    pub fingerprint: String,
    pub password: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RefreshTokens {
    pub fingerprint: String,
}
