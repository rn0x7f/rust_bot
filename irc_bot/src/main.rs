mod bot;
mod osu_api;
use crate::bot::message_handler::MessageHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handler = MessageHandler::new().await?;
    handler.listen().await?;
    Ok(())
}