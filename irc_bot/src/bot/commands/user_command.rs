use crate::bot::simple_message;

pub async fn user_command(
    client: &irc::client::Client,
    nickname: &str,
    osu_api_client: &crate::osu_api::OsuApiClient,
    username: &str
) -> Result<(), anyhow::Error> {
    let user = osu_api_client.get_user_data(username).await?; // El `?` ahora funciona bien

    // Construir un solo mensaje con todos los datos, usando un delimitador en lugar de saltos de línea
    let message = format!(
        "User: {} | ID: {} | Country: {} | Playcount: {} | Ranked Score: {} | Total Score: {} | PP: {} | Accuracy: {} | Level: {} | Rank: {} | Country Rank: {}",
        user.username,
        user.user_id,
        user.country,
        user.playcount,
        user.ranked_score,
        user.total_score,
        user.pp_raw,
        user.accuracy,
        user.level,
        user.pp_rank,
        user.pp_country_rank
    );

    // Enviar el mensaje completo
    simple_message::simple_message(client, nickname, &message).await;

    Ok(())
}
