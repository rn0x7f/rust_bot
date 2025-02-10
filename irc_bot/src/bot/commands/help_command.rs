use crate::bot::simple_message;

pub async fn help_command(client: &irc::client::Client, nickname: &str) {
    // Construir el mensaje completo con los comandos, usando un delimitador
    let message = "Available commands: | !help - Check the help panel | !user {username} - Check the user's data | !req {something else} - View the message you sent";

    // Enviar el mensaje completo
    simple_message::simple_message(client, nickname, message).await;
}
