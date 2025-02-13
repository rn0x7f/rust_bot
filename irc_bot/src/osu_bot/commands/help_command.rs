use crate::osu_bot::functions::simple_message;

pub async fn help_command(client: &irc::client::Client, nickname: &str) {
    let message = "Available commands: | !help - Check the help panel | !user {username} - Check the user's data | !req {something else} - View the message you sent";

    simple_message::simple_message(client, nickname, message).await;
}
