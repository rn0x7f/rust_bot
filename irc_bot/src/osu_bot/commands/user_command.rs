use crate::osu_bot::functions::simple_message;

pub async fn user_command(
    client: &irc::client::Client,
    nickname: &str,
    osu_api_client: &mut crate::osu_api_client::OsuAPIClient,
    username: &str
) -> Result<(), anyhow::Error> {
    let user = osu_api_client.get_user(username).await?;

    let message = format!(
        "User: {} | ID: {} | Country: {} | Playcount: {} | Total Hits: {} | PP: {} | Accuracy: {} | Rank: {} | Play Time: {}s | Active: {}",
        user.username,
        user.id,
        user.country_code,
        user.statistics.play_count,
        user.statistics.total_hits,
        user.statistics.pp,
        user.statistics.hit_accuracy,
        user.statistics.global_rank.unwrap_or(-1),
        user.statistics.play_time,
        if user.is_active { "Yes" } else { "No" }
    );

    simple_message::simple_message(client, nickname, &message).await;

    Ok(())
}
