use crate::osu_bot::functions::command_handler;
use crate::osu_api_client::OsuAPIClient;

use irc::client::Client;
use irc::proto::Command;
use futures::StreamExt;

pub async fn listen(irc_client: &mut Client, osu_api_client: &mut OsuAPIClient,bot_nickname: String) -> Result<(), anyhow::Error> {
    let stream = &mut irc_client.stream()?;
        while let Some(message) = stream.next().await.transpose()? {
            match message.command {
                Command::PRIVMSG(ref target, ref msg) => {
                    let author = message.source_nickname().unwrap_or("unknown");
                    if *target == bot_nickname {
                        if msg.starts_with('!') {
                            command_handler::handle_command(irc_client, osu_api_client, author, msg).await;
                        } else {

                            println!("[{}]: {}", author, msg);
                        }
                    }
                }
                Command::PING(ref token, ref optional_msg) => {
                    println!("🔄 Recibido PING de servidor: {:?}, {:?}", token, optional_msg);
                    irc_client.send(Command::PONG(token.clone(), None))?; // Agrega `None` como segundo argumento
                    println!("✅ Enviado PONG correctamente");
                }
                _ => {}
            }
        }
        Ok(())
}