use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize};
use askama::Template;
use crate::GameAppData;
use actix_web::web::Data;
use core::{Club, Team};

#[derive(Deserialize)]
pub struct TeamGetRequest {
    team_id: u32
}

#[derive(Template)]
#[template(path = "teams/get/get.html")]
pub struct TeamGetViewModel<'c> {
    pub id: u32,
    pub name: &'c str, 
    pub balance: TeamBalance,
    pub players: Vec<TeamPlayer<'c>>
}

pub struct TeamBalance{
    pub amount: i32,
    pub income: i32,
    pub outcome: i32
}

pub struct TeamPlayer<'cp>{
    pub id: u32,
    pub last_name: &'cp str,
    pub first_name: &'cp str,
}

pub async fn team_get_action(state: Data<GameAppData>, route_params: web::Path<TeamGetRequest>) -> Result<HttpResponse> {
    let guard = state.data.lock();

    let simulator_data = guard.as_ref().unwrap();

    let team: &Team = simulator_data.continents.iter().flat_map(|c| &c.countries)      
        .flat_map(|c| &c.clubs)
        .flat_map(|c| &c.teams)
        .find(|t| t.id == route_params.team_id)
        .unwrap();

    let model = TeamGetViewModel {
        id: team.id,
        name: &team.name,
        balance: TeamBalance {
            amount: 0,
            income: 0,
            outcome: 0
        },
        players: team.players().iter().map(|p| {
            TeamPlayer {
                id: p.id,
                first_name: &p.full_name.first_name,
                last_name: &p.full_name.last_name
            }
        }).collect()
    };

    let html = TeamGetViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
