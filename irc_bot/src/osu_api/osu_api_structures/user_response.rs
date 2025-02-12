use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Statistics {
    pub global_rank: Option<i64>, // Puede ser null, así que usa Option<i64>
    pub pp: f64,
    pub total_hits: i64,
    pub hit_accuracy: f64,
    pub play_count: i64,
    pub play_time: i64,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub username: String,
    pub country_code: String,
    pub id: i64,
    pub is_active: bool,
    pub statistics: Statistics,
}
