use crate::GameAppData;
use actix_web::web;
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct MatchDetailsRequest {
    pub league_id: u32,
    pub match_id: String,
}

#[derive(Serialize)]
pub struct MatchDetailsResponse {
    pub position_data: HashMap<u32, Vec<(u64, i16, i16)>>,
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

    let players_data = &match_details.details.as_ref().unwrap().position_data;

    Json(MatchDetailsResponse {
        position_data: players_data
            .data
            .iter()
            .map(|(&player_id, data)| {
                (
                    player_id,
                    data.iter()
                        .map(|item| (item.timestamp, item.x, item.y))
                        .collect(),
                )
            })
            .collect(),
    })
}
