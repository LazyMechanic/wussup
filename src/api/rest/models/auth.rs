#[derive(serde::Deserialize, Debug)]
pub struct LoginRequest {
    pub fingerprint: String,
    pub password: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RefreshTokensRequest {
    pub fingerprint: String,
}

#[derive(serde::Serialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(serde::Serialize, Debug)]
pub struct RefreshTokensResponse {
    pub access_token: String,
}
