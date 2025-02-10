use serde::Serialize;

#[derive(Serialize)]
pub struct UserRequest {
    pub k: String,       // API key
    pub u: String,       // Nombre de usuario o user_id
    pub m: Option<u8>,   // Modo de juego (opcional, por defecto 0)
    pub type_: Option<String>, // Tipo de parámetro `u` (opcional)
    pub event_days: Option<u8>, // Maximo número de días entre el evento y ahora (opcional)
}