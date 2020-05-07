use actix_web::{HttpResponse, Result};
use crate::simulator::SimulatorData;
use crate::server::GLOBAL_DATA;
use std::sync::Mutex;
use serde::{Serialize};
use crate::utils::TimeEstimation;

#[derive(Serialize)]
pub struct IndexResponse {
    game_id: String,
    elapsed: u32
}

pub async fn game_index_action() -> Result<HttpResponse> {
    let estimated = TimeEstimation::estimate(SimulatorData::generate);

    let simulator_data = estimated.0;
    
    let game_id = simulator_data.id();

    GLOBAL_DATA.insert(simulator_data.id(), Mutex::new(simulator_data));

    let json_result = serde_json::to_string(&IndexResponse{
        game_id,
        elapsed: estimated.1
    }).unwrap();

    Ok(HttpResponse::Ok().body(json_result))
}
