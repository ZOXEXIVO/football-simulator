use crate::GameAppData;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use askama::Template;
use core::{SimulatorData, Team};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TeamScheduleGetRequest {
    team_slug: String,
}

#[derive(Template)]
#[template(path = "teams/schedule/schedule.html")]
pub struct TeamScheduleViewModel<'t> {
    pub team_name: &'t str,
    pub league_id: u32,
    pub league_name: &'t str,
    pub neighbor_teams: Vec<ClubTeam<'t>>,
    pub items: Vec<TeamScheduleItem<'t>>,
}

pub struct TeamScheduleItem<'t> {
    pub date: String,
    pub time: String,
    pub opponent_slug: &'t str,
    pub opponent_name: &'t str,
    pub is_home: bool,
    pub competition_id: u32,
    pub competition_name: &'t str,
    pub result: Option<TeamScheduleItemResult>,
}

pub struct TeamScheduleItemResult {
    pub home_goals: i32,
    pub away_goals: i32,
}

pub struct ClubTeam<'c> {
    pub slug: &'c str,
    pub name: &'c str,
    pub reputation: u16,
}

pub async fn team_schedule_get_action(
    state: Data<GameAppData>,
    route_params: web::Path<TeamScheduleGetRequest>,
) -> Result<HttpResponse> {
    let guard = state.data.lock().await;

    let simulator_data = guard.as_ref().unwrap();

    let team_id = simulator_data
        .indexes
        .as_ref()
        .unwrap()
        .slug_indexes
        .get_team_by_slug(&route_params.team_slug)
        .unwrap();

    let team: &Team = simulator_data.team(team_id).unwrap();

    let league = simulator_data.league(team.league_id).unwrap();

    let model = TeamScheduleViewModel {
        team_name: &team.name,

        league_id: league.id,
        league_name: &league.name,
        neighbor_teams: get_neighbor_teams(team.club_id, simulator_data),

        items: league
            .schedule
            .get_matches_for_team(team.id)
            .iter()
            .map(|schedule| {
                let is_home = schedule.home_team_id == team.id;

                let home_team_data = simulator_data.team_data(schedule.home_team_id).unwrap();
                let away_team_data = simulator_data.team_data(schedule.away_team_id).unwrap();

                TeamScheduleItem {
                    date: schedule.date.format("%d.%m.%Y").to_string(),
                    time: schedule.date.format("%H:%M").to_string(),
                    opponent_slug: if is_home {
                        &away_team_data.slug
                    } else {
                        &home_team_data.slug
                    },
                    opponent_name: if is_home {
                        &away_team_data.name
                    } else {
                        &home_team_data.name
                    },
                    is_home,
                    competition_id: league.id,
                    competition_name: &league.name,
                    result: if schedule.result.is_some() {
                        Some(TeamScheduleItemResult {
                            home_goals: schedule.result.as_ref().unwrap().home_goals,
                            away_goals: schedule.result.as_ref().unwrap().away_goals,
                        })
                    } else {
                        None
                    },
                }
            })
            .collect(),
    };

    let html = TeamScheduleViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

fn get_neighbor_teams(club_id: u32, data: &SimulatorData) -> Vec<ClubTeam> {
    let club = data.club(club_id).unwrap();

    let mut teams: Vec<ClubTeam> = club
        .teams
        .teams
        .iter()
        .map(|team| ClubTeam {
            slug: &team.slug,
            name: &team.name,
            reputation: team.reputation.world,
        })
        .collect();

    teams.sort_by(|a, b| b.reputation.cmp(&a.reputation));

    teams
}
