use crate::GameAppData;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use core::utils::TimeEstimation;
use core::FootballSimulator;
use std::sync::Arc;

pub async fn game_process_action(State(state): State<GameAppData>) -> impl IntoResponse {
    let data = Arc::clone(&state.data);

    let mut simulator_data_guard = data.write_owned().await;

    let process_result = tokio::task::spawn_blocking(move || {
        let simulator_data = simulator_data_guard.as_mut().unwrap();

        let (_, estimated) =
            TimeEstimation::estimate(|| FootballSimulator::simulate(simulator_data));

        estimated
    })
    .await;

    let estimated = process_result.unwrap();

    let mut headers = HeaderMap::new();

    headers.insert("Location", "/".parse().unwrap());
    headers.insert("Estimated", estimated.to_string().parse().unwrap());

    (StatusCode::OK, headers, Json(()))
}
