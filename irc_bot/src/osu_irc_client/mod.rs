use irc::client::prelude::*;

pub struct OsuIRCClient {
    host: String,
	port: u16,
	nickname: String,
	password: String,
	autojoin: Vec<String>,
}

impl OsuIRCClient {
    pub fn new(host: String, port: u16, nickname: String, password: String, autojoin: Vec<String>) -> Self {
        Self {
            host,
            port,
            nickname,
            password,
            autojoin,
        }
    }

    pub async fn connect(&self) -> Result<Client, irc::error::Error> {
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