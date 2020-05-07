use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA, ClubDto};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct ClubListRequest {
    game_id: String
}

#[derive(Serialize)]
pub struct ClubListResponse<'c> {
    pub clubs: Vec<ClubDto<'c>>
}

pub async fn club_list_action(route_params: web::Path<ClubListRequest>) -> Result<HttpResponse> {
    let data = GLOBAL_DATA.read().unwrap();

    if !data.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let simulator_data = data.get(&route_params.game_id).unwrap().lock().unwrap();

    let clubs = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .flat_map(|cn| &cn.leagues)
        .flat_map(|l| &l.clubs);
    
    let json_result = serde_json::to_string(&ClubListResponse{
        clubs: clubs.map(|c| ClubDto {
            name: &c.name
        }).collect()
    }).unwrap();
    
    Ok(HttpResponse::Ok().body(json_result))
}
