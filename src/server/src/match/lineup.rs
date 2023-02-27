use crate::GameAppData;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use core::r#match::engine::FootballMatchDetails;
use core::SimulatorData;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MatchDetailsRequest {
    pub league_slug: String,
    pub match_id: String,
}

#[derive(Serialize)]
pub struct MatchLineupResponse<'p> {
    pub home_squad: LineupSquad<'p>,
    pub away_squad: LineupSquad<'p>,
    pub ball: LineupBall,
    pub match_time_ms: u64,
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

#[derive(Serialize)]
pub struct LineupBall {
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
        match_time_ms: match_details.match_time_ms,
        ball: LineupBall {
            start_position: (
                match_details.position_data.ball_positions[0].x as i16,
                match_details.position_data.ball_positions[0].y as i16,
            ),
        },
        home_squad: LineupSquad {
            main: match_details
                .home_players
                .main
                .iter()
                .filter_map(|player_id| {
                    to_lineup_player(*player_id, home_team_slug, match_details, simulator_data)
                })
                .collect(),
            substitutes: match_details
                .home_players
                .substitutes
                .iter()
                .filter_map(|player_id| {
                    to_lineup_player(*player_id, home_team_slug, match_details, simulator_data)
                })
                .collect(),
        },
        away_squad: LineupSquad {
            main: match_details
                .away_players
                .main
                .iter()
                .filter_map(|player_id| {
                    to_lineup_player(*player_id, away_team_slug, match_details, simulator_data)
                })
                .collect(),
            substitutes: match_details
                .away_players
                .substitutes
                .iter()
                .filter_map(|player_id| {
                    to_lineup_player(*player_id, away_team_slug, match_details, simulator_data)
                })
                .collect(),
        },
    };

    Json(result).into_response()
}

fn to_lineup_player<'p>(
    player_id: u32,
    team_slug: &'p str,
    match_details: &'p FootballMatchDetails,
    simulator_data: &'p SimulatorData,
) -> Option<LineupPlayer<'p>> {
    let player = simulator_data.player(player_id).unwrap();

    let position = match_details.position_data.player_positions.get(&player_id);

    match position {
        Some(position) => {
            let position = position.first().unwrap();

            Some(LineupPlayer {
                id: player.id,
                first_name: &player.full_name.first_name,
                last_name: &player.full_name.last_name,
                middle_name: player.full_name.middle_name.as_deref(),
                position: player.position().get_short_name(),
                team_slug,
                start_position: (position.x as i16, position.y as i16),
            })
        }
        None => None,
    }
}
