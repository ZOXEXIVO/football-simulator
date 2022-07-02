use crate::GameAppData;
use actix_web::error::BlockingError;
use actix_web::http::header::REFERER;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, Result};
use core::utils::TimeEstimation;
use core::FootballSimulator;

pub async fn game_process_action(
    request: HttpRequest,
    state: Data<GameAppData>,
) -> Result<HttpResponse> {
    let mut data = state.data.lock().await;

    let process_result: Result<u32, BlockingError> = actix_web::web::block(move || {
        let simulator_data = data.as_mut().unwrap();

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
