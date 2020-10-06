use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize};
use askama::Template;
use crate::GameAppData;
use actix_web::web::Data;
use core::Club;

#[derive(Deserialize)]
pub struct ClubGetRequest {
    club_id: u32,
}

#[derive(Template)]
#[template(path = "clubs/get/get.html")]
pub struct ClubGetViewModel<'c> {
    pub id: u32,
    pub name: &'c str, 
    pub balance: ClubBalance,
    pub players: Vec<ClubPlayer<'c>>
}

pub struct ClubBalance{
    pub amount: i32,
    pub income: i32,
    pub outcome: i32
}

pub struct ClubPlayer<'cp>{
    pub id: u32,
    pub last_name: &'cp str,
    pub first_name: &'cp str,
}

pub async fn club_get_action(state: Data<GameAppData>, route_params: web::Path<ClubGetRequest>) -> Result<HttpResponse> {
    let guard = state.data.lock();

    let mut simulator_data = guard.as_ref().unwrap();

    let club: &Club = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .flat_map(|cn| &cn.leagues)
        .flat_map(|l| &l.clubs)
        .find(|club| club.id == route_params.club_id)
        .unwrap();

    let model = ClubGetViewModel {
        id: club.id,
        name: &club.name,
        balance: ClubBalance {
            amount: club.finance.balance.amount,
            income: club.finance.balance.income,
            outcome: club.finance.balance.outcome
        },
        players: club.players().iter().map(|p| {
            ClubPlayer {
                id: p.id,
                first_name: &p.full_name.first_name,
                last_name: &p.full_name.last_name
            }
        }).collect()
    };

    let html = ClubGetViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
