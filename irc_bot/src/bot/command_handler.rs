use irc::client::Client;
use crate::osu_api::OsuApiClient;
use crate::bot::simple_message;
use crate::bot::commands::help_command;
use crate::bot::commands::user_command;

pub async fn handle_command(client: &Client, message: &irc::client::prelude::Message, msg: &str, osu_api_client: &OsuApiClient) {
    let parts: Vec<&str> = msg.split_whitespace().collect(); // Separar el mensaje en partes
    if let Some(command) = parts.get(0) {
        // Remover el "!" al principio del comando
        let command = &command[1..];
        // Verificar si el comando existe
        match command {
            "help" => {
                help_command::help_command(client, message.source_nickname().unwrap_or("unknown")).await;
            }
            "user" => {
                if parts.len() != 2 {
                    simple_message::simple_message(client, message.source_nickname().unwrap_or("unknown"), "Usage: !user <username>").await;
                } else {
                    let username = parts[1..].join(" ");
                    match user_command::user_command(client, message.source_nickname().unwrap_or("unknown"), osu_api_client, &username).await {
                        Ok(_) => {}
                        Err(e) => {
                            simple_message::simple_message(client, message.source_nickname().unwrap_or("unknown"), &format!("Error: {}", e)).await;
                        }
                    }
                }
            }
            "req" => {
                let prompt = parts[1..].join(" ");
                println!("Prueba: {}", prompt);
                simple_message::simple_message(client, message.source_nickname().unwrap_or("unknown"), &format!("Dijiste: {}", prompt)).await;
            }
            _ => {
                // Si el comando no existe
                simple_message::simple_message(client, message.source_nickname().unwrap_or("unknown"), "Unknown command. Please use !help for assistance.").await;
            }
        }
    }
}