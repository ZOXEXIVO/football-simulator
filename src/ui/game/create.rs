use crate::db::Generator;
use crate::GameAppData;
use actix_web::error::BlockingError;
use actix_web::web::Data;
use actix_web::{HttpResponse, Result};
use core::utils::TimeEstimation;

pub async fn game_create_action(state: Data<GameAppData>) -> Result<HttpResponse> {
    let process_result: Result<u32, BlockingError> = actix_web::web::block(move || {
        let (generated_data, estimated) =
            TimeEstimation::estimate(|| Generator::generate(&state.database));

        let mut data = state.data.lock();

        *data = Some(generated_data);

        estimated
    })
    .await;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .append_header(("Estimated", process_result.unwrap().to_string()))
        .finish())
}
