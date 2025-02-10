use irc::client::Client;
use crate::simple_message;

pub fn handle_command(client: &Client, message: &irc::client::prelude::Message, msg: &str) {
    let parts: Vec<&str> = msg.split_whitespace().collect(); // Separar el mensaje en partes
    if let Some(command) = parts.get(0) {
        // Remover el "!" al principio del comando
        let command = &command[1..];
        // Verificar si el comando existe
        match command {
            "help" => {
                simple_message::simple_message(client, message.source_nickname().unwrap_or("unknown"), "Espera we aun no hay nada");
            }
            "req" => {
                let prompt = parts[1..].join(" ");
                println!("Prueba: {}", prompt);
                simple_message::simple_message(client, message.source_nickname().unwrap_or("unknown"), &format!("Dijiste: {}", prompt));                                }
            _ => {
                // Si el comando no existe
                simple_message::simple_message(client, message.source_nickname().unwrap_or("unknown"), "Comando desconocido, usa !help");
            }
        }
    }
}