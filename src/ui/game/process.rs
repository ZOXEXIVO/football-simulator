use crate::GameAppData;
use actix_web::error::BlockingError;
use actix_web::http::header::REFERER;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, Result};
use core::utils::TimeEstimation;
use core::FootballSimulator;
use std::sync::Arc;

pub async fn game_process_action(
    request: HttpRequest,
    state: Data<GameAppData>,
) -> Result<HttpResponse> {
    let data = Arc::clone(&state.data);

    let mut simulator_data_guard = data.lock_owned().await;

    let process_result: Result<u32, BlockingError> = actix_web::web::block(move || {
        let simulator_data = simulator_data_guard.as_mut().unwrap();

        let (_, estimated) =
            TimeEstimation::estimate(|| FootballSimulator::simulate(simulator_data));

        estimated
    })
    .await;

    let referrer = request.headers().get(REFERER).unwrap();

    Ok(HttpResponse::Found()
        .append_header(("Location", referrer.to_str().unwrap_or("/")))
        .append_header(("Estimated", process_result.unwrap().to_string()))
        .finish())
}
