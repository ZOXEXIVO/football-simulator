use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LeagueGetRequest {
    game_id: String
}

#[derive(Serialize)]
pub struct LeagueGetResponse {
    //pub name: &'l str
}

pub async fn league_get_action(route_params: web::Path<LeagueGetRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let simulator_data = GLOBAL_DATA.get(&route_params.game_id).unwrap();

    let leagues = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .flat_map(|cn| &cn.leagues);
    
    
    
    let result = LeagueGetResponse{
        
    };
    
    Ok(HttpResponse::Ok().json(result))
}
