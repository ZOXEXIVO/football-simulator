use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA, LeagueDto};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LeagueListRequest {
    game_id: String
}

#[derive(Serialize)]
pub struct LeagueListResponse<'c> {
    pub leagues: Vec<LeagueDto<'c>>
}

pub async fn league_list_action(route_params: web::Path<LeagueListRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let data = GLOBAL_DATA.get(&route_params.game_id).unwrap();

    let simulator_data = data.lock().unwrap();

    let leagues = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .flat_map(|cn| &cn.leagues);
    
    let json_result = serde_json::to_string(&LeagueListResponse{
        leagues: leagues.map(|c| LeagueDto {
            name: &c.name
        }).collect()
    }).unwrap();
    
    Ok(HttpResponse::Ok().body(json_result))
}
