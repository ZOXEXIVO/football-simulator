use crate::GameAppData;
use async_compression::tokio::write::GzipEncoder;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use core::utils::TimeEstimation;
use core::FootballSimulator;
use core::SimulationResult;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::stores::MatchStore;

pub async fn game_process_action(State(state): State<GameAppData>) -> impl IntoResponse {
    let data = Arc::clone(&state.data);

    let mut simulator_data_guard = data.write_owned().await;

    let game_processing_result = tokio::task::spawn_blocking(move || {
        let simulator_data = simulator_data_guard.as_mut().unwrap();

        TimeEstimation::estimate(|| FootballSimulator::simulate(simulator_data))
    })
    .await;

    let mut headers = HeaderMap::new();

    headers.insert("Location", "/".parse().unwrap());

    if let Ok((result, estimated)) = game_processing_result {
        write_match_results(result).await;

        headers.insert("Estimated", estimated.to_string().parse().unwrap());

        return (StatusCode::OK, headers, Json(()));
    }

    (StatusCode::BAD_REQUEST, headers, Json(()))
}


async fn write_match_results(result: SimulationResult) {
    for match_result in result.match_results {
        MatchStore::store(match_result).await;
    }
}
