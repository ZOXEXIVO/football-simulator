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

#[derive(Serialize)]
pub struct MatchLineupResponse<'p> {
    pub home_squad: LineupSquad<'p>,
    pub away_squad: LineupSquad<'p>,
}

#[derive(Serialize)]
pub struct LineupSquad<'p> {
    pub main: Vec<LineupPlayer<'p>>,
    pub substitutes: Vec<LineupPlayer<'p>>,
}

#[derive(Serialize)]
pub struct LineupPlayer<'p> {
    pub id: u32,
    pub first_name: &'p str,
    pub last_name: &'p str,
    pub middle_name: Option<&'p str>,
    pub position: &'p str,
    pub team_slug: &'p str,
    pub start_position: (i16, i16),
}

pub async fn match_lineup_action(
    State(state): State<GameAppData>,
    Path(route_params): Path<MatchDetailsRequest>,
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

    let home_team_slug = &simulator_data
        .team(match_details.home_team_id)
        .unwrap()
        .slug;

    let away_team_slug = &simulator_data
        .team(match_details.away_team_id)
        .unwrap()
        .slug;

    let match_details = match_details.details.as_ref().unwrap();

    let result = MatchLineupResponse {
        home_squad: LineupSquad {
            main: match_details
                .home_team_players
                .iter()
                .map(|player_id| {
                    let player = simulator_data.player(*player_id).unwrap();
                    let position = match_details
                        .position_data
                        .player_positions
                        .get(player_id)
                        .unwrap()
                        .first()
                        .unwrap();

                    LineupPlayer {
                        id: player.id,
                        first_name: &player.full_name.first_name,
                        last_name: &player.full_name.last_name,
                        middle_name: player.full_name.middle_name.as_deref(),
                        position: player.position().get_short_name(),
                        team_slug: home_team_slug,
                        start_position: (position.x as i16, position.y as i16),
                    }
                })
                .collect(),
            substitutes: Vec::new(),
        },
        away_squad: LineupSquad {
            main: match_details
                .away_team_players
                .iter()
                .map(|player_id| {
                    let player = simulator_data.player(*player_id).unwrap();
                    let position = match_details
                        .position_data
                        .player_positions
                        .get(player_id)
                        .unwrap()
                        .first()
                        .unwrap();

                    LineupPlayer {
                        id: player.id,
                        first_name: &player.full_name.first_name,
                        last_name: &player.full_name.last_name,
                        middle_name: player.full_name.middle_name.as_deref(),
                        position: player.position().get_short_name(),
                        team_slug: away_team_slug,
                        start_position: (position.x as i16, position.y as i16),
                    }
                })
                .collect(),
            substitutes: Vec::new(),
        },
    };

    Json(result).into_response()
}
