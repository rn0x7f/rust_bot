use irc::client::Client;
use crate::bot::simple_message;
use crate::bot::commands::help_command;

pub fn handle_command(client: &Client, message: &irc::client::prelude::Message, msg: &str) {
    let parts: Vec<&str> = msg.split_whitespace().collect(); // Separar el mensaje en partes
    if let Some(command) = parts.get(0) {
        // Remover el "!" al principio del comando
        let command = &command[1..];
        // Verificar si el comando existe
        match command {
            "help" => {
                help_command::help_command(client, message.source_nickname().unwrap_or("unknown"));
            }
            "req" => {
                let prompt = parts[1..].join(" ");
                println!("Prueba: {}", prompt);
                simple_message::simple_message(client, message.source_nickname().unwrap_or("unknown"), &format!("Dijiste: {}", prompt));                                }
            _ => {
                // Si el comando no existe
                simple_message::simple_message(client, message.source_nickname().unwrap_or("unknown"), "Unknown command. Please use !help for assistance.");
            }
        }
    }
}