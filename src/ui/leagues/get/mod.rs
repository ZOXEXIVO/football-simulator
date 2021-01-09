use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize};
use askama::Template;
use actix_web::web::Data;
use core::context::NaiveDateTime;
use itertools::*;
use core::league::ScheduleTour;
use crate::GameAppData;

#[derive(Deserialize)]
pub struct LeagueGetRequest {
    league_id: u32,
}

#[derive(Template)]
#[template(path = "leagues/get/get.html")]
pub struct LeagueGetViewModel<'l> {
    pub id: u32,
    pub name: &'l str,
    pub country_id: u32,
    pub country_name: &'l str,
    pub table: LeagueTableDto<'l>,
    pub current_tour_schedule: Vec<TourSchedule<'l>>
}

pub struct TourSchedule<'s> {
    pub date: String,
    pub matches: Vec<LeagueScheduleItem<'s>>
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

    let now = simulator_data.date.date();
    
    let league = simulator_data.league(route_params.league_id).unwrap();

    let country = simulator_data.country(league.country_id).unwrap();
    
    let league_table = league.table.as_ref().unwrap().get();
       
    let mut model = LeagueGetViewModel {
        id: league.id,
        name: &league.name,
        country_id: country.id,
        country_name: &country.name,
        table: LeagueTableDto {
            rows: league_table.iter().map(|t| LeagueTableRow {
                team_id: t.team_id,
                team_name: simulator_data.team_name(t.team_id).unwrap(),
                played: t.played,
                win: t.win,
                draft: t.draft,
                lost: t.lost,
                goal_scored: t.goal_scored,
                goal_concerned: t.goal_concerned,
                points: t.points,
            }).collect()
        },
        current_tour_schedule: Vec::new()
    };

    let actual_tour: Vec<&ScheduleTour> = league.schedule.tours
        .iter()
        .map(|t| (t, t.min_date()))
        .filter_map(|(tour, min_date)| {
            if !tour.played && min_date.is_some() && min_date.unwrap() > now {
                return Some(tour);
            }          
            
            None
        })
        .take(1)
        .collect();    

    
    match actual_tour.first() {
        Some(tour) => {
            for (key, group) in &tour.items.iter().group_by(|t| t.date.date()) {
                let tour_schedule = TourSchedule {
                    date: key.format("%d.%m.%Y").to_string(),
                    matches: group.map(|item| {
                        LeagueScheduleItem {
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
                            home_team_name: simulator_data.team_name(item.home_team_id).unwrap(),

                            away_team_id: item.away_team_id,
                            away_team_name: simulator_data.team_name(item.away_team_id).unwrap(),
                        }
                    }).collect()
                };
                  
                model.current_tour_schedule.push(tour_schedule)
            }
        },
        None => {
           
        }
    }
    
    let html = LeagueGetViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
