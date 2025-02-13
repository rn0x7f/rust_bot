pub mod functions;
// pub mod commands;

use crate::osu_api_client::OsuAPIClient;
use crate::osu_irc_client::OsuIRCClient;
use crate::config::Config;
use functions::listen;

pub struct OsuBot {
    osu_api_client: OsuAPIClient,
    osu_irc_client: OsuIRCClient,
    config: Config,
}

impl OsuBot {
    pub fn new() -> Self {
        let config = Config::new();
        let osu_api_client = OsuAPIClient::new(config.osu_client_id.try_into().unwrap(), config.osu_client_secret.to_string());
        let osu_irc_client = OsuIRCClient::new(
            config.irc_server.to_string(), 
            config.irc_port.try_into().unwrap(), 
            config.irc_nickname.to_string(), 
            config.irc_password.to_string(), 
            config.irc_channels.clone()
        );
        Self {
            osu_api_client,
            osu_irc_client,
            config,
        }
    }

    pub async fn authenticate(&mut self) -> Result<(), anyhow::Error> {
        self.osu_api_client.authenticate().await?;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), anyhow::Error> {
        let mut client = self.osu_irc_client.connect().await?;
        println!("Connected to IRC server");
        client.identify()?;
        println!("Identified with IRC server");
        listen::listen(&mut client, self.config.irc_nickname.to_string()).await?;
        Ok(())
    }
}