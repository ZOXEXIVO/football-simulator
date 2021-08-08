use crate::db::Generator;
use crate::GameAppData;
use actix_web::web::Data;
use actix_web::{HttpResponse, Result};
use core::utils::TimeEstimation;
use actix_web::error::BlockingError;

pub async fn game_create_action(state: Data<GameAppData>) -> Result<HttpResponse> {
    let process_result: Result<u32, BlockingError<&str>> = actix_web::web::block(move || {
        let (generated_data, estimated) =
            TimeEstimation::estimate(|| Generator::generate(&state.database));

        let mut data = state.data.lock();

        *data = Some(generated_data);

        Ok(estimated)
    }).await;
    
    Ok(HttpResponse::Found()
        .header("Location", "/")
        .header("Estimated", process_result.unwrap().to_string())
        .finish())
}
