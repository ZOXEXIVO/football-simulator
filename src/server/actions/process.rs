use actix_web::{web, HttpResponse, Result};
use crate::simulator::{FootballSimulator};
use crate::server::GLOBAL_DATA;
use serde::{Serialize, Deserialize};
use crate::utils::TimeEstimation;

#[derive(Deserialize)]
pub struct ProcessRequest {
    game_id: String
}

#[derive(Serialize)]
pub struct ProcessResponse {
    game_id: String,
    elapsed: u32
}

pub async fn process_action(route_params: web::Path<ProcessRequest>) -> Result<HttpResponse> {
    let mut global_data = GLOBAL_DATA.write().unwrap();

    let state = &mut *global_data;
    
    if !state.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let mut simulator_data = state.get_mut(&route_params.game_id).unwrap().lock().unwrap(); 

    let estimated = TimeEstimation::estimate(||
        FootballSimulator::simulate(&mut simulator_data)
    ); 

    let json_result = serde_json::to_string(&ProcessResponse{
        game_id: simulator_data.id(),
        elapsed: estimated.1
    }).unwrap();
    
    Ok(HttpResponse::Ok().body(json_result))
}
