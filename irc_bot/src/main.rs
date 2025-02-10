mod bot;
mod osu_api;
use crate::bot::message_handler::MessageHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handler = MessageHandler::new().await?;
    handler.listen().await?;
    Ok(())
}

// mod osu_api; // Asegúrate de que todos los módulos se incluyan aquí

// #[tokio::main]
// async fn main() {
//     let osu_api_client = osu_api::OsuApiClient::new();
//     // Crear una nueva instancia de OsuApiClient

//     // El nombre de usuario que quieres consultar
//     let username = "Xoerix";  // Cambia esto por el nombre de usuario deseado

//     // Obtener los datos del usuario
//     match osu_api_client.get_user_data(username).await {
//         Ok(user) => {
//             println!("Usuario encontrado: {} ({})", user.username, user.country); // Cambiar `country_code` por `country`
//         }
//         Err(e) => {
//             eprintln!("Error al obtener los datos del usuario: {}", e);
//         }
//     }
// }
