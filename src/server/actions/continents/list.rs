use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA, ContinentDto};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct ContinentListRequest {
    game_id: String
}

#[derive(Serialize)]
pub struct ContinentListResponse<'c> {
    pub continents: Vec<ContinentDto<'c>>
}

pub async fn continent_list_action(route_params: web::Path<ContinentListRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let data = GLOBAL_DATA.get(&route_params.game_id).unwrap();

    let simulator_data = data.lock().unwrap();

    let json_result = serde_json::to_string(&ContinentListResponse{
        continents: simulator_data.continents.iter().map(|c| ContinentDto {
            name: &c.name
        }).collect()
    }).unwrap();
    
    Ok(HttpResponse::Ok().body(json_result))
}
