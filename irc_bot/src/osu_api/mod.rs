use reqwest::Client;
use anyhow::Context;
pub mod api_structures;
pub mod credentials;
pub mod osu_api_requests;
use crate::osu_api::api_structures::auth_request;
use crate::osu_api::api_structures::auth_response;
use crate::osu_api::api_structures::user_response;

use tokio::time::{Duration, Instant};

#[derive(Clone)]
pub struct OsuApiClient {
    pub client_id: i64,
    pub client_secret: String,
    pub access_token: String,
    pub expires_at: Instant, // Guardamos el tiempo de expiración absoluto
    pub client: Client,
}

impl Default for OsuApiClient {
    fn default() -> Self {
        let credentials = credentials::APICredentials::new();
        Self {
            client_id: credentials.osu_client_id,
            client_secret: credentials.osu_client_secret,
            access_token: "".to_string(),
            expires_at: Instant::now(), // Se inicia en el momento actual
            client: Client::new(),
        }
    }
}

impl OsuApiClient {
    pub async fn authenticate(&mut self) -> Result<String, anyhow::Error> {
        let auth = auth_request::AuthRequest {
            client_id: self.client_id,
            client_secret: self.client_secret.clone(),
            grant_type: "client_credentials".to_string(),
            scope: "public".to_string(),
        };

        let url = "https://osu.ppy.sh/oauth/token";

        let res = self.client
            .post(url)
            .json(&auth)
            .send()
            .await
            .context("Error al enviar la solicitud")?;

        let body: auth_response::AuthResponse = res.json().await.context("Error al deserializar la respuesta")?;

        self.access_token = body.access_token;
        self.expires_at = Instant::now() + Duration::from_secs(body.expires_in as u64);

        Ok(self.access_token.clone())
    }

    pub async fn ensure_authenticated(&mut self) -> Result<(), anyhow::Error> {
        if Instant::now() >= self.expires_at {
            println!("Token expirado, autenticando...");
            self.authenticate().await?;
        }
        Ok(())
    }

    pub async fn get_user(&mut self, username: &str) -> Result<user_response::UserResponse, anyhow::Error> {
        self.ensure_authenticated().await?;
        osu_api_requests::get_user::get_user(&self.client, &self.access_token, username).await
    }
}
