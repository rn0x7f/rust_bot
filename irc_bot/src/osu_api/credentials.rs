use dotenvy;
use std::env;

pub(crate) struct APICredentials {
    pub osu_client_id: i64,
    pub osu_client_secret: String,
}

impl APICredentials {
    pub fn new() -> Self {
        dotenvy::from_filename("config/.env").ok();

        Self {
            osu_client_id: env::var("OSU_CLIENT_ID").expect("OSU_CLIENT_ID not set").parse().expect("OSU_CLIENT_ID is not a number"),
            osu_client_secret: env::var("OSU_CLIENT_SECRET").expect("OSU_CLIENT_SECRET not set"),
        }
    }
}