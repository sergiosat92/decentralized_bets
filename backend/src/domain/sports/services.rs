use crate::{
    domain::sports::{
        dtos::{ErrorResponse, GetAllLeaguesResponse},
        sports::{Leagues, LeaguesApiResponse, API_AUTH_HEADER, API_BASE_URL, API_KEY},
    },
    infrastructure::web::{cache::CACHE, http_client::send_request},
};
use axum::http::StatusCode;
use axum::Json;
use reqwest::Method;

static INDEX_LEAGUES: u8 = 1;

pub async fn get_leagues(
) -> Result<(StatusCode, Json<GetAllLeaguesResponse>), (StatusCode, Json<ErrorResponse>)> {
    match CACHE.get(&INDEX_LEAGUES) {
        Some(leagues_str) => {
            let leagues: Vec<Leagues> = serde_json::from_str(&leagues_str).unwrap_or(vec![]);
            match leagues.len() > 0 {
                false => match get_leagues_from_api().await {
                    Ok(leagues) => {
                        CACHE.insert(INDEX_LEAGUES, serde_json::to_string(&leagues).unwrap());
                        Ok((StatusCode::OK, Json(GetAllLeaguesResponse { leagues })))
                    }
                    Err(e) => Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse { message: e }),
                    )),
                },
                true => Ok((StatusCode::OK, Json(GetAllLeaguesResponse { leagues }))),
            }
        }
        None => match get_leagues_from_api().await {
            Ok(leagues) => {
                CACHE.insert(INDEX_LEAGUES, serde_json::to_string(&leagues).unwrap());
                Ok((StatusCode::OK, Json(GetAllLeaguesResponse { leagues })))
            }
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: e }),
            )),
        },
    }
}

async fn get_leagues_from_api() -> Result<Vec<Leagues>, String> {
    let url = format!("{}/leagues{}{}", API_BASE_URL, API_AUTH_HEADER, API_KEY);
    match send_request::<(), LeaguesApiResponse>(&url, Method::GET, None, None, None).await {
        Ok(Some(leagues_response)) => Ok(leagues_response.data),
        Ok(None) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
