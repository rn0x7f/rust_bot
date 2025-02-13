use irc::client::Client;
use crate::{bot::{commands::{help_command, user_command}, simple_message},osu_api::OsuApiClient};

pub async fn handle_command(client: &Client, message: &irc::client::prelude::Message, msg: &str, osu_api_client: &mut OsuApiClient) {
    let parts: Vec<&str> = msg.split_whitespace().collect(); // Separar el mensaje en partes
    if let Some(command) = parts.get(0) {
        // Remover el "!" al principio del comando
        let command = &command[1..];
        // Verificar si el comando existe
        let source_nickname = message.source_nickname().unwrap_or("unknown");

        match command {
            "help" => {
                help_command::help_command(client, source_nickname).await;
            }
            "user" => {
                if parts.len() != 2 {
                    simple_message::simple_message(client, source_nickname, "Usage: !user <username>").await;
                } else {
                    let username = parts[1..].join(" ");
                    match user_command::user_command(client, source_nickname, osu_api_client, &username).await {
                        Ok(_) => {}
                        Err(e) => {
                            simple_message::simple_message(client, source_nickname, &format!("Error: {}", e)).await;
                        }
                    }
                }
            }
            "req" => {
                let prompt = parts[1..].join(" ");
                println!("Prueba: {}", prompt);
                simple_message::simple_message(client, source_nickname, &format!("Dijiste: {}", prompt)).await;
            }
            _ => {
                // Si el comando no existe
                simple_message::simple_message(client, source_nickname, "Unknown command. Please use !help for assistance.").await;
            }
        }
    }
}