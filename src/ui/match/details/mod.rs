use crate::GameAppData;
use actix_web::web;
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MatchDetailsRequest {
    pub league_id: u32,
    pub match_id: String,
}

#[derive(Serialize)]
pub struct MatchDetailsResponse {
    pub players_data: Vec<PlayerPositionDataDto>,
}

#[derive(Serialize)]
pub struct PlayerPositionDataDto {
    player_id: u32,
    x: i16,
    y: i16,
    timestamp: u64,
}

pub async fn match_details_action(
    state: Data<GameAppData>,
    route_params: web::Path<MatchDetailsRequest>,
) -> Json<MatchDetailsResponse> {
    let guard = state.data.lock().await;

    let simulator_data = guard.as_ref().unwrap();

    let league = simulator_data.league(route_params.league_id).unwrap();

    let match_details = league
        .match_results
        .iter()
        .find(|m| m.id == route_params.match_id)
        .unwrap();

    let players_data = match_details
        .details
        .as_ref()
        .unwrap()
        .players_positions
        .iter()
        .map(|p| PlayerPositionDataDto {
            player_id: 1,
            x: p.x,
            y: p.y,
            timestamp: p.timestamp,
        })
        .collect();

    Json(MatchDetailsResponse { players_data })
}
