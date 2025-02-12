use dotenvy::dotenv;
use std::env;

pub struct Config {
    // Osu! APIv2
    pub osu_client_id: i32,
    pub osu_client_secret: String,
    // IRC
    pub irc_server: String,
    pub irc_nickname: String,
    pub irc_password: String,
    pub irc_port: u16,
    pub irc_channels: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            // Osu! APIv2
            osu_client_id: env::var("OSU_CLIENT_ID").expect("OSU_CLIENT_ID not set").parse().expect("OSU_CLIENT_ID is not a number"),
            osu_client_secret: env::var("OSU_CLIENT_SECRET").expect("OSU_CLIENT_SECRET not set"),
            irc_server: env::var("IRC_SERVER").expect("IRC_SERVER not set"),
            // IRC
            irc_nickname: env::var("IRC_NICKNAME").expect("IRC_NICKNAME not set"),
            irc_password: env::var("IRC_PASSWORD").expect("IRC_PASSWORD not set"),
            irc_port: env::var("IRC_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(6667),
            irc_channels: env::var("IRC_CHANNELS")
                .ok()
                .map(|s| s.split(',').map(|c| c.trim().to_string()).collect())
                .unwrap_or_else(|| vec!["#osu".to_string()]),
        }
    }
}