mod message_handler;
mod connection;
mod credentials;
mod simple_message;
mod command_handler;
use crate::message_handler::MessageHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handler = MessageHandler::new().await?;
    handler.listen().await?;
    Ok(())
}
