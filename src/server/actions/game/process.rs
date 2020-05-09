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

pub async fn game_process_action(route_params: web::Path<ProcessRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }
    
    let data= GLOBAL_DATA.get_mut(&route_params.game_id).unwrap();

    let mut simulator_data = data.lock().unwrap(); 

    let estimated = TimeEstimation::estimate(||
        FootballSimulator::simulate(&mut simulator_data)
    ); 

    let result = ProcessResponse{
        game_id: simulator_data.id(),
        elapsed: estimated.1
    };
    
    Ok(HttpResponse::Ok().json(result))
}
