use dotenvy;
use std::env;

pub(crate) struct APICredentials {
    pub osu_api_key: String,
}

impl APICredentials {
    pub fn new() -> Self {
        dotenvy::from_filename("config/.env").ok();

        Self {
            osu_api_key: env::var("OSU_API_KEY").expect("OSU_API_KEY not set"),
        }
    }
}