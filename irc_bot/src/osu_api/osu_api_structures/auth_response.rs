use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub expires_in: i64,
}