use serde::Serialize;

use crate::domain::sports::sports::Leagues;

#[derive(Serialize)]
pub struct GetAllLeaguesResponse {
    pub leagues: Vec<Leagues>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}
