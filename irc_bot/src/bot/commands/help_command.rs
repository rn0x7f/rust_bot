use crate::bot::simple_message;

pub fn help_command(client: &irc::client::Client, nickname: &str) {
    simple_message::simple_message(client, nickname, "Available commands:");
    simple_message::simple_message(client, nickname, "!help - Check the help panel");
    simple_message::simple_message(client, nickname, "!req {something else} - View the message you sent");
}