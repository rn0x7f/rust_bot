use crate::bot::simple_message;
use crate::osu_api::OsuApiClient;

pub async fn user_command(
    client: &irc::client::Client,
    nickname: &str,
    osu_api_client: &mut OsuApiClient,
    username: &str
) -> Result<(), anyhow::Error> {
    let user = osu_api_client.get_user(username).await?;

    let message = format!(
        "User: {} | ID: {} | Country: {} | Playcount: {} | Play Time: {} | PP: {:.2} | Accuracy: {:.2}% | Total hits {} ] Global Rank: {} ] Active: {}",
        user.username,
        user.id,
        user.country_code,
        user.statistics.play_count,
        user.statistics.play_time,
        user.statistics.pp,
        user.statistics.hit_accuracy,
        user.statistics.total_hits,
        user.statistics.global_rank.unwrap_or(0), // `unwrap_or(0)` para evitar problemas con `null`
        user.is_active,
    );

    // Enviar el mensaje completo
    simple_message::simple_message(client, nickname, &message).await;

    Ok(())
}
