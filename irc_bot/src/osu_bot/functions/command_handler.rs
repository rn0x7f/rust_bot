use crate::osu_api_client::OsuAPIClient;

use irc::client::Client;

use crate::osu_bot::functions::simple_message;
use crate::osu_bot::commands::help_command;
// use crate::osu_bot::commands::user_command;


pub async fn handle_command(irc_client: &mut Client, osu_api_client: &mut OsuAPIClient, author: &str, msg: &str) {
    let parts: Vec<&str> = msg.split_whitespace().collect(); // Separar el mensaje en partes
    if let Some(command) = parts.get(0) {
        // Remover el "!" al principio del comando
        let command = &command[1..];
        // Verificar si el comando existe
        match command {
            "help" => {
                help_command::help_command(irc_client, author).await;
            }
            // "user" => {
            //     if parts.len() != 2 {
            //         simple_message::simple_message(client, author, "Usage: !user <username>").await;
            //     } else {
            //         let username = parts[1..].join(" ");
            //         match user_command::user_command(client, author, osu_api_client, &username).await {
            //             Ok(_) => {}
            //             Err(e) => {
            //                 simple_message::simple_message(client, author, &format!("Error: {}", e)).await;
            //             }
            //         }
            //     }
            // }
            "req" => {
                let prompt = parts[1..].join(" ");
                println!("Prueba: {}", prompt);
                simple_message::simple_message(irc_client, author, &format!("Dijiste: {}", prompt)).await;
            }
            _ => {
                // Si el comando no existe
                simple_message::simple_message(irc_client, author, "Unknown command. Please use !help for assistance.").await;
            }
        }
    }
}