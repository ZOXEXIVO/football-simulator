use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA};
use serde::{Serialize, Deserialize};
use crate::league::ScheduleItem;

#[derive(Deserialize)]
pub struct LeagueGetRequest {
    game_id: String,
    league_id: u32
}

#[derive(Serialize)]
pub struct LeagueGetResponse<'l> {
   league: LeagueDto<'l>
}

#[derive(Serialize)]
pub struct LeagueDto<'l> {
    pub id: u32,
    pub name: &'l str,
    pub table: LeagueTableDto<'l>,
    pub week_schedule: Option<LeagueSchedule<'l>>
}

#[derive(Serialize)]
pub struct LeagueSchedule<'s> {
    pub items: Vec<LeagueScheduleItem<'s>>
}

#[derive(Serialize)]
pub struct LeagueScheduleItem<'si> {
    pub home_goals: Option<u8>,
    pub away_goals: Option<u8>,
    
    pub home_club_id: u32,
    pub home_club_name: &'si str,
    
    pub guest_club_id: u32,
    pub guest_club_name: &'si str,
}

#[derive(Serialize)]
pub struct LeagueTableDto<'l> {
    pub rows: Vec<LeagueTableRow<'l>>
}

#[derive(Serialize)]
pub struct LeagueTableRow<'l> {
    pub club_id: u32,
    pub club_name: &'l str,
    pub played: u8,
    pub win: u8,
    pub draft: u8,
    pub lost: u8,
    pub goal_scored: u8,
    pub goal_concerned: u8,
    pub points: u8,
}

pub async fn league_get_action(route_params: web::Path<LeagueGetRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let simulator_data = GLOBAL_DATA.get(&route_params.game_id).unwrap();

    let league = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .flat_map(|cn| &cn.leagues)
        .find(|l| l.id == route_params.league_id).unwrap();
    
    let league_table = league.table.get();
    
    let leagues_schedule: Option<LeagueSchedule> = match &league.schedule {
        Some(schedule) => {
            // if schedule.current_tour.is_none() {
            //     None
            // }

            None
            // else {
            //     Some(LeagueSchedule {
            //         items: schedule.current_tour.unwrap().games.map(|g| LeagueScheduleItem {
            //             pub home_goals: Option<u8>,
            //         pub away_goals: Option<u8>,
            //
            //         pub home_club_id: u32,
            //         pub home_club_name: &'si str,
            //
            //         pub guest_club_id: u32,
            //         pub guest_club_name: &'si str,
            //         })
            //     })
            // }           
        },
        None => None
    };
    
    let result = LeagueGetResponse{
        league: LeagueDto {
            id: league.id,
            name: &league.name,
            table: LeagueTableDto {
                rows: league_table.iter().map(|t| LeagueTableRow {
                    club_id: t.club_id,
                    club_name: &league.clubs.iter().find(|c| c.id == t.club_id).unwrap().name,
                    played: t.played,
                    win: t.win,
                    draft: t.draft,
                    lost: t.lost,
                    goal_scored: t.goal_scored,
                    goal_concerned: t.goal_concerned,
                    points: t.points
                }).collect()
            },
            week_schedule: leagues_schedule
        }
    };
    
    Ok(HttpResponse::Ok().json(result))
}
