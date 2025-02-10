use reqwest::Client;
use anyhow::{Result, Context};
use crate::osu_api::api_structures::osu_user::OsuUser;
use crate::osu_api::api_structures::user_request::UserRequest;

pub async fn get_user_data(api_key: &str, username: &str) -> Result<OsuUser> {
    let client = Client::new();
    let url = "https://osu.ppy.sh/api/get_user";

    // Construir los parámetros de la solicitud
    let params = UserRequest {
        k: api_key.to_string(),
        u: username.to_string(),
        m: Some(0),  // Modo 0 (osu!)
        type_: Some("string".to_string()),  // Indica que `u` es un nombre de usuario
        event_days: Some(1),  // Obtener eventos dentro de 1 día
    };

    let res = client
        .get(url)
        .query(&params)
        .send()
        .await
        .context("Error al enviar la solicitud")?;

    let body = res.text().await.context("Error al leer la respuesta del servidor")?;
    //println!("Respuesta cruda: {}", body);  // Ddepuración

    let user_data: Vec<OsuUser> = serde_json::from_str(&body)
        .context("Error al deserializar la respuesta JSON")?;

    if let Some(user) = user_data.first() {
        Ok(user.clone())
    } else {
        Err(anyhow::anyhow!("No se encontraron datos del usuario").into())
    }
}
