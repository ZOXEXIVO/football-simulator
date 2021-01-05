use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize};
use askama::Template;
use crate::GameAppData;
use actix_web::web::Data;
use core::{Team, SimulatorData};

#[derive(Deserialize)]
pub struct TeamScheduleGetRequest {
    team_id: u32
}

#[derive(Template)]
#[template(path = "teams/schedule/schedule.html")]
pub struct TeamScheduleViewModel<'t>  {
    pub team_name: &'t str,
    pub league_id: u32,
    pub league_name: &'t str,
    pub neighbor_teams: Vec<ClubTeam<'t>>,
    pub items: Vec<TeamScheduleItem<'t>>
}

pub struct TeamScheduleItem<'t> {
    pub date: String,
    pub time: String,
    pub opponent_id: u32,
    pub opponent_name: &'t str,
    pub is_home: bool,
    pub competition_id: u32,
    pub competition_name: &'t str    
}

pub struct ClubTeam<'c>{
    pub id: u32,
    pub name: &'c str,
    pub reputation: u16
}

pub async fn team_schedule_get_action(state: Data<GameAppData>, route_params: web::Path<TeamScheduleGetRequest>) -> Result<HttpResponse> {
    let guard = state.data.lock();

    let simulator_data = guard.as_ref().unwrap();

    let team: &Team = simulator_data.team(route_params.team_id).unwrap();

    let league = simulator_data.league(team.league_id).unwrap();
    
    let model = TeamScheduleViewModel {
        team_name: &team.name,
        
        league_id: league.id,
        league_name: &league.name,
        neighbor_teams: get_neighbor_teams(team.club_id, simulator_data),
        
        items: league.schedule.get_matches_for_team(team.id).iter().map(|schedule| {
            let is_home = schedule.home_team_id == team.id;

            TeamScheduleItem {
                date: schedule.date.format("%d.%m.%Y").to_string(),
                time: schedule.date.format("%H:%M").to_string(),
                opponent_id: if is_home {
                    schedule.away_team_id
                } else {
                    schedule.home_team_id
                },
                opponent_name: if is_home {
                    &simulator_data.team(schedule.away_team_id).unwrap().name
                } else {
                    &simulator_data.team(schedule.home_team_id).unwrap().name
                },
                is_home,
                competition_id: league.id,
                competition_name: &league.name               
            }
        }).collect()
    };

    let html = TeamScheduleViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

fn get_neighbor_teams(club_id: u32, data: &SimulatorData) -> Vec<ClubTeam> {
    let club = data.club(club_id).unwrap();

    let mut teams: Vec<ClubTeam> = club.teams.iter().map(|team| {
        ClubTeam {
            id: team.id,
            name: &team.name,
            reputation: team.reputation.world
        }
    }).collect();

    teams.sort_by(|a, b| b.reputation.cmp(&a.reputation));

    teams
}