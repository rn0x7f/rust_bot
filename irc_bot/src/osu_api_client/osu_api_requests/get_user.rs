use reqwest::Client;
use anyhow::{Result, Context};
use crate::osu_api_client::osu_api_structures::user_response;

pub async fn get_user(client: &Client, url: &str, access_token: &str, username: &str) -> Result<user_response::UserResponse> {
    let url = format!("{}/api/v2/users/{}", url, username);
    
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .context("Error al enviar la solicitud")?;

    let body: user_response::UserResponse = res.json().await.context("Error al deserializar la respuesta")?;
    Ok(body)
}