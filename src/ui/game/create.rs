use actix_web::{HttpResponse, Result};
use std::sync::{Mutex, Arc};
use core::{SimulatorData};
use actix_web::web::Data;
use crate::GameAppData;
use std::borrow::BorrowMut;
use core::utils::TimeEstimation;

pub async fn game_create_action(state: Data<GameAppData>) -> Result<HttpResponse> {
    let (generated_data, estimated) = TimeEstimation::estimate(SimulatorData::generate);

    let mut data = state.data.lock().unwrap();

    *data = Some(generated_data);

    Ok(HttpResponse::Found()
        .header("Location", "/")
        .header("Estimated", estimated.to_string())
        .finish()
    )
}
