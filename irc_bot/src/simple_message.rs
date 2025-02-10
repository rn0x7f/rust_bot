// src/message_handler.rs

use irc::client::Client;

pub fn simple_message(client: &Client, nickname: &str, msg: &str) {
    // Enviamos un mensaje privado al usuario que nos escribió
    if let Err(e) = client.send_privmsg(nickname, msg) {
        eprintln!("Error al enviar mensaje: {}", e);
    } else {
        println!("Xoerix -> {}: {}", nickname, msg);
    }
}
