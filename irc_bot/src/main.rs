mod osu_bot;
mod config;
mod osu_api_client;
mod osu_irc_client;
mod llm_client;

use osu_bot::OsuBot;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut bot = OsuBot::new();
    bot.authenticate().await?;
    bot.connect().await?;
    Ok(())
}

// mod llm_client;
// use crate::llm_client::LLMClient;  // Importar LLMClient desde el archivo llm_client.rs
// use anyhow::Result;  // Importar Result desde anyhow

// #[tokio::main]
// async fn main() -> Result<()> {
//     // Instanciamos el cliente con la URL de la API
//     let llm_client = LLMClient::new("http://localhost:11434/api/generate".to_string());

//     // Llamamos a la función ask_prompt con el prompt
//     let response = llm_client.ask_prompt("Platicame algo y usa saltos de linea con \n".to_string()).await?;

//     // Imprimir solo el mensaje final sin saltos de línea
//     println!("Respuesta: {}", response);

//     Ok(())
// }
