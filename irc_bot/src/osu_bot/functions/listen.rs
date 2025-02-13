use irc::client::Client;
use irc::proto::Command;
use futures::StreamExt;

pub async fn listen(client: &mut Client, bot_nickname: String) -> Result<(), anyhow::Error> {
    let stream = &mut client.stream()?;
        while let Some(message) = stream.next().await.transpose()? {
            match message.command {
                Command::PRIVMSG(ref target, ref msg) => {
                    let author = message.source_nickname().unwrap_or("unknown");
                    if *target == bot_nickname {
                        if msg.starts_with('!') {
                            println!("[{}]: comando-{}", author, msg);
                        } else {
                            println!("[{}]: {}", author, msg);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
}