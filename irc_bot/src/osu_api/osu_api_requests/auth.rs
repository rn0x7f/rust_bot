use reqwest::Client;
use anyhow::{Result, Context};
use tokio::time::Instant;
use crate::osu_api::osu_api_structures::{auth_response, auth_request};

pub async fn auth(client: Client, main_url: &str, client_id: i32, client_secret: String) -> Result<(String, Instant), anyhow::Error> {
    let auth: auth_request::AuthRequest = auth_request::AuthRequest {
        client_id,
        client_secret,
        grant_type: "client_credentials".to_string(),
        scope: "public".to_string(),
    };

    let url = format!("{}/oauth/token", main_url);

    let res = client
        .post(&url)
        .json(&auth)
        .send()
        .await
        .context("Error al enviar la solicitud")?;

    let body: auth_response::AuthResponse = res.json().await.context("Error al deserializar la respuesta")?;

    Ok((body.access_token, Instant::now() + tokio::time::Duration::from_secs(body.expires_in as u64)))
}