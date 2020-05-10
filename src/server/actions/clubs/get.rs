use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA};
use serde::{Serialize, Deserialize};
use crate::club::Club;

#[derive(Serialize)]
pub struct ClubDto<'c> {
    pub id: u32,
    pub name: &'c str,
    pub balance: ClubBalance
}

#[derive(Serialize)]
pub struct ClubBalance {
    pub amount: i32,
    pub income: i32,
    pub outcome: i32,
}

#[derive(Deserialize)]
pub struct ClubGetRequest {
    game_id: String,
    club_id: u32
}

#[derive(Serialize)]
pub struct ClubGetResponse<'c> {
    pub club: ClubDto<'c>
}

pub async fn club_get_action(route_params: web::Path<ClubGetRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let data = GLOBAL_DATA.get(&route_params.game_id).unwrap();

    let simulator_data = data.lock().unwrap();

    let club: &Club = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .flat_map(|cn| &cn.leagues)
        .flat_map(|l| &l.clubs)
        .find(|club| club.id == route_params.club_id)
        .unwrap();

    let result = ClubGetResponse {
        club: ClubDto {
            id: club.id,
            name: &club.name,
            balance: ClubBalance {
                amount: club.finance.balance.amount,
                income: club.finance.balance.income,
                outcome: club.finance.balance.outcome
            }
        }
    };

    Ok(HttpResponse::Ok().json(result))
}
