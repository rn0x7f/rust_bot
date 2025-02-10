use irc::client::prelude::*;
use crate::bot::credentials::IRCCredentials;

pub struct Connection {
	pub host: String,
	pub port: u16,
	pub nickname: String,
	pub password: String,
	pub autojoin: Vec<String>,
}

impl Default for Connection {
    fn default() -> Self {
        let credentials = IRCCredentials::new();
        Self {
            host: credentials.irc_server,
            port: credentials.irc_port,
            nickname: credentials.irc_nickname,
            password: credentials.irc_password,
            autojoin: credentials.irc_channels,
        }
    }
}

impl Connection {
    pub(crate) async fn connect(&self) -> Result<Client, irc::error::Error> {
        println!("Connecting to {}:{} as {}", self.host, self.port, self.nickname);
        let config = Config {
            nickname: Some(self.nickname.clone()),
            server: Some(self.host.clone()),
            port: Some(self.port),
            password: Some(self.password.clone()), // Pueden ser nulos o no
            channels: self.autojoin.clone(), // Siempre es un vector, este vacio o no
            use_tls: Some(false),
            ..Config::default()
        };
        Ok(Client::from_config(config).await?)
    }
}