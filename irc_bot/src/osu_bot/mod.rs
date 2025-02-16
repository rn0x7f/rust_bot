pub mod functions;
pub mod commands;

use crate::osu_api_client::OsuAPIClient;
use crate::osu_irc_client::OsuIRCClient;
use crate::config::Config;
use functions::listen;
use crate::llm_client::LLMClient;

pub struct OsuBot {
    osu_api_client: OsuAPIClient,
    osu_irc_client: OsuIRCClient,
    llm_client: LLMClient,
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
        let llm_client = LLMClient::new(config.llm_api_url.to_string());
        Self {
            osu_api_client,
            osu_irc_client,
            llm_client,
            config,
        }
    }

    pub async fn authenticate(&mut self) -> Result<(), anyhow::Error> {
        self.osu_api_client.authenticate().await?;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), anyhow::Error> {
        let mut irc_client = self.osu_irc_client.connect().await?;
        println!("Connected to IRC server");
        irc_client.identify()?;
        println!("Identified with IRC server");
        let osu_api_client = &mut self.osu_api_client;
        listen::listen(&mut irc_client, osu_api_client, &mut self.llm_client, self.config.irc_nickname.to_string()).await?;
        Ok(())
    }
}