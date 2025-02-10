use reqwest::Client;
use anyhow::{Result, Context};
use crate::osu_api::api_structures::user_response;

pub async fn get_user(client: &Client, access_token: &str, username: &str) -> Result<user_response::UserResponse> {
    let url = format!("https://osu.ppy.sh/api/v2/users/{}", username);
    
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .context("Error al enviar la solicitud")?;

    let body: user_response::UserResponse = res.json().await.context("Error al deserializar la respuesta")?;
    Ok(body)
}