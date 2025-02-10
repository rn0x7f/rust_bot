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
            if let Command::PRIVMSG(ref target, ref msg) = message.command {
                // Mensajes privados al bot
                if target == &self.nickname {
                    // Verificar si el mensaje comienza con "!", si es un comando
                    if msg.starts_with('!') {
                        // Manejar el comando
                        command_handler::handle_command(&self.client, &message, msg);
                    } else {
                        // Si no es un comando, responder con un mensaje simple
                        println!("[{}]: {}", message.source_nickname().unwrap_or("unknown"), msg);
                        simple_message::simple_message(&self.client, message.source_nickname().unwrap_or("unknown"), "Eso no es un comando flaco");
                    }
                }
                // Resto de mensajes a canales
                // ...
            }
        }
        Ok(())
    }
}
