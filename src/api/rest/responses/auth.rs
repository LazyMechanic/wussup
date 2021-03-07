#[derive(serde::Serialize, Debug)]
pub struct Login {
    pub access_token: String,
}

#[derive(serde::Serialize, Debug)]
pub struct RefreshTokens {
    pub access_token: String,
}
