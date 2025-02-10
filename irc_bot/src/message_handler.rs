use crate::connection::Connection;
use irc::proto::Command;
use irc::client::Client;
use futures_util::StreamExt;
use crate::simple_message;
use crate::command_handler;

pub struct MessageHandler {
    client: Client,
    nickname: String,
}

impl MessageHandler {
    pub async fn new() -> Result<Self, irc::error::Error> {
        let connection = Connection::default();
        let client = connection.connect().await.unwrap();
        println!("Connected to IRC server");
        client.identify()?; // Asegura que el bot se autentique correctamente
        let nickname = connection.nickname.clone();
        Ok(Self { client, nickname }) 
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
                            command_handler::handle_command(&self.client, &message, msg);
                        } else {
                            // Si no es un comando, responder con un mensaje simple
                            println!("[{}]: {}", message.source_nickname().unwrap_or("unknown"), msg);
                            simple_message::simple_message(&self.client, message.source_nickname().unwrap_or("unknown"), "Eso no es un comando flaco");
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
