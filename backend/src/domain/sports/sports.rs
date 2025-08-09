use serde::{Deserialize, Serialize};

pub static API_KEY: &str = "g7E3SZYM5wsQFc3W9yvkIz1KTK8bdCLsNo9ZrNxt9Bh0cv3uMJ9sg2BA6eRQ";
pub static API_BASE_URL: &str = "https://cricket.sportmonks.com/api/v2.0";
pub static API_AUTH_HEADER: &str = "?api_token=";

#[derive(Deserialize, Debug)]
pub struct LeaguesApiResponse {
    pub data: Vec<Leagues>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leagues {
    pub resource: String,
    pub id: u32,
    pub season_id: u32,
    pub country_id: u32,
    pub name: String,
    pub code: String,
    pub image_path: String,

    #[serde(rename = "type")]
    pub league_type: String,
    pub updated_at: String,
}
