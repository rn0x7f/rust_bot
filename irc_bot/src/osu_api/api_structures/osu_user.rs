use serde::Deserialize;

#[derive(Clone)]
#[allow(dead_code)] // No siempre se usan todos los campos
#[derive(Deserialize, Debug)]
pub struct OsuUser {
    pub user_id: String,
    pub username: String,
    pub country: String,
    pub join_date: String,
    pub count300: String,
    pub count100: String,
    pub count50: String,
    pub playcount: String,
    pub ranked_score: String,
    pub total_score: String,
    pub pp_rank: String,
    pub level: String,
    pub pp_raw: String,
    pub accuracy: String,
    pub count_rank_ss: String,
    pub count_rank_ssh: String,
    pub count_rank_s: String,
    pub count_rank_sh: String,
    pub count_rank_a: String,
    pub total_seconds_played: String,
    pub pp_country_rank: String,
}