use crate::db::Generator;
use crate::{GameAppData, SimulatorData};
use actix_web::error::BlockingError;
use actix_web::web::Data;
use actix_web::{HttpResponse, Result};
use core::utils::TimeEstimation;

pub async fn game_create_action(state: Data<GameAppData>) -> Result<HttpResponse> {
    let mut state_data = state.data.lock().await;

    let cloned_state = Data::clone(&state);

    let generation_result: Result<(SimulatorData, u32), BlockingError> =
        actix_web::web::block(move || {
            let (generated_data, estimated) =
                TimeEstimation::estimate(|| Generator::generate(&cloned_state.database));

            (generated_data, estimated)
        })
        .await;

    let (data, estimated) = generation_result.unwrap();

    *state_data = Some(data);

    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .append_header(("Estimated", estimated.to_string()))
        .finish())
}
