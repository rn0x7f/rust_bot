mod osu_bot;
mod config;
mod osu_api_client;
mod osu_irc_client;

use osu_bot::OsuBot;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut bot = OsuBot::new();
    bot.authenticate().await?;
    bot.connect().await?;
    Ok(())
}