use crate::GameAppData;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use axum::http::StatusCode;

#[derive(Deserialize)]
pub struct MatchDataRequest {
    pub league_slug: String,
    pub match_id: String,
}

#[derive(Serialize)]
pub struct MatchDetailsResponse {
    pub player_data: HashMap<u32, Vec<(u64, i16, i16, i16)>>,
    pub player_data_len: u32,
    pub ball_data: Vec<(u64, i16, i16, i16)>,
}

pub async fn match_get_action(
    State(state): State<GameAppData>,
    Path(route_params): Path<MatchDataRequest>
) -> Response {
    let guard = state.data.read().await;

    let simulator_data = guard.as_ref().unwrap();

    let league_id = simulator_data
        .indexes
        .as_ref()
        .unwrap()
        .slug_indexes
        .get_league_by_slug(&route_params.league_slug)
        .unwrap();

    let league = simulator_data.league(league_id).unwrap();

    let match_result = league.matches.get(route_params.match_id).unwrap();

    let result_details = match_result.result_details.as_ref().unwrap();

    if let Some(result_details) = match_result.result_details.as_ref() {

        let mut response = (StatusCode::OK, result_details.position_data.clone())
            .into_response();

        response.headers_mut().append("Content-Type", "application/json".parse().unwrap());
        response.headers_mut().append("Content-Encoding", "gzip".parse().unwrap());

        return response;
    }

    (StatusCode::NOT_FOUND, result_details.position_data.clone())
        .into_response()
}
