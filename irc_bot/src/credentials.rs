use dotenvy;
use std::env;

pub(crate) struct Credentials {
    pub irc_server: String,
    pub irc_nickname: String,
    pub irc_password: String,
    pub irc_port: u16,
    pub irc_channels: Vec<String>,
}

impl Credentials {
    pub fn new() -> Self {
        dotenvy::from_filename("config/.env").ok();

        Self {
            irc_server: env::var("IRC_SERVER").expect("IRC_SERVER not set"),
            irc_nickname: env::var("IRC_NICKNAME").expect("IRC_NICKNAME not set"),
            irc_password: env::var("IRC_PASSWORD").expect("IRC_PASSWORD not set"),
            irc_port: env::var("IRC_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(6667), // Valor por defecto
            irc_channels: env::var("IRC_CHANNELS")
                .ok()
                .map(|s| s.split(',').map(|c| c.trim().to_string()).collect())
                .unwrap_or_else(|| vec!["#osu".to_string()]), // Valor por defecto
        }
    }
}
