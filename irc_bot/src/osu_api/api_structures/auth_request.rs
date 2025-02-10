use serde::Serialize;

#[derive(Serialize)]
pub struct AuthRequest {
    pub client_id: i64,
    pub client_secret: String,
    pub grant_type: String,
    pub scope: String,
}