use crate::osu_bot::functions::simple_message;

pub async fn help_command(client: &irc::client::Client, nickname: &str) {
    let message = "!help | !user {username} | !ask {prompt} | !req {something else}";

    simple_message::simple_message(client, nickname, message).await;
}
