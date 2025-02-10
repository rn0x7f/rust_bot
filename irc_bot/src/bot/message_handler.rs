use irc::proto::Command;
use irc::client::Client;
use futures_util::StreamExt;
use crate::bot::connection::Connection;
use crate::bot::simple_message;
use crate::bot::command_handler;
use crate::osu_api::OsuApiClient;

pub struct MessageHandler {
    client: Client,
    osu_api_client: OsuApiClient,
    nickname: String,
}

impl MessageHandler {
    pub async fn new() -> Result<Self, irc::error::Error> {
        let connection = Connection::default();
        let client = connection.connect().await?;
        println!("Connected to IRC server");
        client.identify()?; // Asegura que el bot se autentique correctamente
        let nickname = connection.nickname.clone();
        let osu_api_client = OsuApiClient::default();
        //let _ = osu_api_client.authenticate().await;
        Ok(Self { client, nickname, osu_api_client }) 
    }

    pub async fn listen(&mut self) -> Result<(), irc::error::Error> {
        let stream = &mut self.client.stream()?;
        while let Some(message) = stream.next().await.transpose()? {
            match message.command {
                Command::PRIVMSG(ref target, ref msg) => {
                    // Mensajes privados al bot
                    if target == &self.nickname {
                        if msg.starts_with('!') {
                            // Manejar el comando
                            command_handler::handle_command(&self.client, &message, msg, &mut self.osu_api_client).await;
                        } else {
                            // Si no es un comando, responder con un mensaje simple
                            println!("[{}]: {}", message.source_nickname().unwrap_or("unknown"), msg);
                            simple_message::simple_message(&self.client, message.source_nickname().unwrap_or("unknown"), "Eso no es un comando flaco").await;
                        }
                    }
                    // Mensajes en canales
                    // ...
                },
                Command::PING(ref server, ref msg) => {
                    // Responder a PING con PONG para mantener la conexión
                    println!("Received PING from server: {}", server);
                    let pong_message = Command::PONG(server.to_string(), msg.clone());
                    self.client.send(pong_message)?;
                    println!("Sent PONG to server");
                },
                _ => {}
            }
        }
        Ok(())
    }
}
