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
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

#[derive(Serialize)]
pub struct MatchDetailsResponse {
    pub player_data: HashMap<u32, Vec<(u64, i16, i16, i16)>>,
    pub player_data_len: u32,
    pub ball_data: Vec<(u64, i16, i16, i16)>,
}

pub async fn match_get_action(
    State(state): State<GameAppData>,
    Path(route_params): Path<MatchDetailsRequest>,
    Query(query_params): Query<MatchDetailsRequestQuery>,
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

    println!(
        "Query, {} {}",
        query_params.start_timestamp, query_params.end_timestamp
    );

    let result = MatchDetailsResponse {
        player_data: result_details
            .position_data
            .player_positions
            .iter()
            .map(|(&player_id, data)| {
                (
                    player_id,
                    data.iter()
                        .filter(|pp| {
                            pp.timestamp >= query_params.start_timestamp
                                && pp.timestamp < query_params.end_timestamp
                        })
                        .map(|item| {
                            (
                                item.timestamp,
                                item.position.x as i16,
                                item.position.y as i16,
                                item.position.z as i16,
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
        player_data_len: result_details.position_data.player_positions.len() as u32,
        ball_data: result_details
            .position_data
            .ball_positions
            .iter()
            .filter(|pp| {
                pp.timestamp >= query_params.start_timestamp
                    && pp.timestamp < query_params.end_timestamp
            })
            .map(|item| {
                (
                    item.timestamp,
                    item.position.x as i16,
                    item.position.y as i16,
                    item.position.z as i16,
                )
            })
            .collect(),
    };

    Json(result).into_response()
}
