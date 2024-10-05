use crate::GameAppData;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use core::SimulatorData;
use serde::{Deserialize, Serialize};

pub async fn match_get_action(
    State(state): State<GameAppData>,
    Path(route_params): Path<MatchGetRequest>,
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

    let home_team = simulator_data.team(match_result.home_team_id).unwrap();
    let away_team = simulator_data.team(match_result.away_team_id).unwrap();

    let result_details = match_result.details.as_ref().unwrap();

    let result = MatchGetResponse {
        score: LineupScore {
            home_goals: result_details.score.home_team.get(),
            away_goals: result_details.score.away_team.get(),
        },
        match_time_ms: result_details.match_time_ms,
        home_team_name: &home_team.name,
        home_team_slug: &home_team.slug,
        home_squad: LineupSquad {
            main: result_details
                .left_team_players
                .main
                .iter()
                .filter_map(|player_id| to_lineup_player(*player_id, simulator_data))
                .collect(),
            substitutes: result_details
                .left_team_players
                .substitutes
                .iter()
                .filter_map(|player_id| to_lineup_player(*player_id, simulator_data))
                .collect(),
        },
        away_team_name: &away_team.name,
        away_team_slug: &away_team.slug,
        away_squad: LineupSquad {
            main: result_details
                .right_team_players
                .main
                .iter()
                .filter_map(|player_id| to_lineup_player(*player_id, simulator_data))
                .collect(),
            substitutes: result_details
                .right_team_players
                .substitutes
                .iter()
                .filter_map(|player_id| to_lineup_player(*player_id, simulator_data))
                .collect(),
        },
    };

    Json(result).into_response()
}

fn to_lineup_player(
    player_id: u32,
    simulator_data: &SimulatorData,
) -> Option<LineupPlayer> {
    let player = simulator_data.player(player_id)?;

    Some(LineupPlayer {
        id: player.id,
        first_name: &player.full_name.first_name,
        last_name: &player.full_name.last_name,
        middle_name: player.full_name.middle_name.as_deref(),
        position: player.position().get_short_name(),
    })
}


#[derive(Deserialize)]
pub struct MatchGetRequest {
    pub league_slug: String,
    pub match_id: String,
}

#[derive(Serialize)]
pub struct MatchGetResponse<'p> {
    // home
    pub home_team_name: &'p str,
    pub home_team_slug: &'p str,
    pub home_squad: LineupSquad<'p>,

    // away
    pub away_team_name: &'p str,
    pub away_team_slug: &'p str,
    pub away_squad: LineupSquad<'p>,

    pub match_time_ms: u64,

    pub score: LineupScore,
}

#[derive(Serialize)]
pub struct LineupScore {
    pub home_goals: u8,
    pub away_goals: u8,
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
}
