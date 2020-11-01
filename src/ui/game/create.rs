use actix_web::{HttpResponse, Result};
use actix_web::web::Data;
use crate::GameAppData;
use core::utils::TimeEstimation;
use crate::db::Generator;

pub async fn game_create_action(state: Data<GameAppData>) -> Result<HttpResponse> {
    let (generated_data, estimated) = TimeEstimation::estimate(||
        Generator::generate(&state.database)
    );

    let mut data = state.data.lock();
    
    *data = Some(generated_data);

    Ok(HttpResponse::Found()
        .header("Location", "/")
        .header("Estimated", estimated.to_string())
        .finish()
    )
}
