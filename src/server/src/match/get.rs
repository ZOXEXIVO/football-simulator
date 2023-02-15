use crate::GameAppData;
use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct MatchDetailsRequest {
    pub league_slug: String,
    pub match_id: String,
}

#[derive(Deserialize)]
pub struct MatchDetailsRequestQuery {
    pub offset: u32,
    pub limit: u32,
}

#[derive(Serialize)]
pub struct MatchDetailsResponse {
    pub player_data: HashMap<u32, Vec<(u64, i16, i16)>>,
    pub player_data_len: u32,
    pub ball_data: Vec<(u64, i16, i16)>,
}

pub async fn match_get_action(
    State(state): State<GameAppData>,
    Path(route_params): Path<MatchDetailsRequest>,
    Query(query_params): Query<MatchDetailsRequestQuery>,
) -> Response {
    let guard = state.data.lock().await;

    let simulator_data = guard.as_ref().unwrap();

    let league_id = simulator_data
        .indexes
        .as_ref()
        .unwrap()
        .slug_indexes
        .get_league_by_slug(&route_params.league_slug)
        .unwrap();

    let league = simulator_data.league(league_id).unwrap();

    let match_details = league
        .match_results
        .iter()
        .find(|m| m.id == route_params.match_id)
        .unwrap();

    let match_details = match_details.details.as_ref().unwrap();

    let result = MatchDetailsResponse {
        player_data: match_details
            .position_data
            .player_positions
            .iter()
            .map(|(&player_id, data)| {
                (
                    player_id,
                    data.iter()
                        .skip(query_params.offset as usize)
                        .take(query_params.limit as usize)
                        .map(|item| (item.timestamp, item.x as i16, item.y as i16))
                        .collect(),
                )
            })
            .collect(),
        player_data_len: match_details.position_data.player_positions.len() as u32,
        ball_data: match_details
            .position_data
            .ball_positions
            .iter()
            .skip(query_params.offset as usize)
            .take(query_params.limit as usize)
            .map(|item| (item.timestamp, item.x as i16, item.y as i16))
            .collect(),
    };

    Json(result).into_response()
}
