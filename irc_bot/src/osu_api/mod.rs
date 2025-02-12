use tokio::time::Instant;
use reqwest::Client;
use crate::osu_api::osu_api_structures::user_response;
use crate::osu_api::osu_api_requests::{auth, get_user};
mod osu_api_structures;
mod osu_api_requests;

pub struct OsuAPI {
    client_id: i32,
    client_secret: String,
    access_token: String,
    expires_at: Instant,
    client: Client,
    main_url: String,
}

impl OsuAPI {
    pub fn new(client_id: i32, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            access_token: "".to_string(),
            expires_at: Instant::now(),
            client: Client::new(),
            main_url: "https://osu.ppy.sh".to_string(),
        }
    }

    pub async fn authenticate(&mut self) -> Result<(), anyhow::Error> {
        println!("Authenticating...");
        let (new_access_token, new_expires_at) = auth::auth(self.client.clone(), &self.main_url, self.client_id, self.client_secret.clone()).await?;
        self.access_token = new_access_token;
        self.expires_at = new_expires_at;
        println!("Authenticated");
        Ok(())
    }

    pub async fn ensure_authenticated(&mut self) -> Result<(), anyhow::Error> {
        if Instant::now() >= self.expires_at {
            println!("Token expired, reauthenticating...");
            self.authenticate().await?;
            println!("Token refreshed");
        }

        Ok(())
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }

    pub async fn get_user(&mut self, username: &String) -> Result<crate::osu_api::osu_api_structures::user_response::UserResponse, anyhow::Error> {
        self.ensure_authenticated().await?;
        let user: user_response::UserResponse = get_user::get_user(&self.client, &self.main_url, &self.access_token, username).await?;
        Ok(user)
    }
}