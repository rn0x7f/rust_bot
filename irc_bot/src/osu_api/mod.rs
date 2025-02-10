pub mod api_structures;
pub mod credentials;
pub mod osu_api_requests;

pub struct OsuApiClient {
    pub credentials: credentials::APICredentials,
}

impl OsuApiClient {
    pub fn new() -> Self {
        Self {
            credentials: credentials::APICredentials::new(),
        }
    }

    // Asegúrate de que la función maneje el tipo de error `anyhow::Error`
    pub async fn get_user_data(&self, username: &str) -> Result<api_structures::osu_user::OsuUser, anyhow::Error> {
        osu_api_requests::get_user_data::get_user_data(&self.credentials.osu_api_key, username).await
    }
}
