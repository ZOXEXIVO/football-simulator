use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize};
use askama::Template;
use crate::GameAppData;
use actix_web::web::Data;
use core::Team;

#[derive(Deserialize)]
pub struct LeagueGetRequest {
    league_id: u32,
}

#[derive(Template)]
#[template(path = "leagues/get/get.html")]
pub struct LeagueGetViewModel<'l> {
    pub id: u32,
    pub name: &'l str,
    pub table: LeagueTableDto<'l>,
    pub week_schedule: LeagueSchedule<'l>,
}

pub struct LeagueSchedule<'s> {
    pub items: Vec<LeagueScheduleItem<'s>>
}

pub struct LeagueScheduleItem<'si> {
    pub home_team_id: u32,
    pub home_team_name: &'si str,

    pub away_team_id: u32,
    pub away_team_name: &'si str,

    pub result: Option<LeagueScheduleItemResult>
}

pub struct LeagueScheduleItemResult {
    pub home_goals: u8,
    pub away_goals: u8,
}

pub struct LeagueTableDto<'l> {
    pub rows: Vec<LeagueTableRow<'l>>
}

pub struct LeagueTableRow<'l> {
    pub team_id: u32,
    pub team_name: &'l str,
    pub played: u8,
    pub win: u8,
    pub draft: u8,
    pub lost: u8,
    pub goal_scored: u8,
    pub goal_concerned: u8,
    pub points: u8,
}

pub async fn league_get_action(state: Data<GameAppData>, route_params: web::Path<LeagueGetRequest>) -> Result<HttpResponse> {
    let guard = state.data.lock();

    let simulator_data = guard.as_ref().unwrap();

    let league = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .flat_map(|cn| &cn.leagues)
        .find(|l| l.id == route_params.league_id).unwrap();

    let teams: Vec<&Team> = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .filter(|cn| cn.id == league.country_id)
        .flat_map(|cn| &cn.clubs)
        .flat_map(|cn| &cn.teams)
        .filter(|t| t.league_id == route_params.league_id)
        .collect();

    let league_table = league.table.as_ref().unwrap().get();
       
    let mut model = LeagueGetViewModel {
        id: league.id,
        name: &league.name,
        table: LeagueTableDto {
            rows: league_table.iter().map(|t| LeagueTableRow {
                team_id: t.team_id,
                team_name: &teams.iter().find(|c| c.id == t.team_id).unwrap().name,
                played: t.played,
                win: t.win,
                draft: t.draft,
                lost: t.lost,
                goal_scored: t.goal_scored,
                goal_concerned: t.goal_concerned,
                points: t.points,
            }).collect()
        },
        week_schedule: LeagueSchedule {
            items: Vec::new()
        },
    };

    for tour in league.schedule.tours.iter().filter(|t| !t.played).take(1) {
        for item in &tour.items {
            let schedule_item = LeagueScheduleItem {
                result: match &item.result {
                    Some(res) => {
                        Some(LeagueScheduleItemResult {
                            home_goals: res.home_goals,
                            away_goals: res.away_goals,
                        })
                    },
                    None => None
                },

                home_team_id: item.home_team_id,
                home_team_name: &teams.iter().find(|c| c.id == item.home_team_id).unwrap().name,

                away_team_id: item.home_team_id,
                away_team_name: &teams.iter().find(|c| c.id == item.away_team_id).unwrap().name,
            };
            
            model.week_schedule.items.push(schedule_item)
        }
    }

    let html = LeagueGetViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
