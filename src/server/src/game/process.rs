use crate::GameAppData;
use async_compression::tokio::write::GzipEncoder;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use core::utils::TimeEstimation;
use core::FootballSimulator;
use core::SimulationResult;
use log::info;
use std::io;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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

const MATCH_DIRECTORY: &str = "matches";

async fn write_match_results(result: SimulationResult) {
    for match_result in result.match_results {
        let out_dir = format!("{}/{}", MATCH_DIRECTORY, match_result.league_slug);

        if let Ok(_) = tokio::fs::create_dir_all(&out_dir).await {
        }

        let out_file = format!("{}/{}", out_dir, match_result.id);

        let file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(out_file)
            .await
            .unwrap();

        let mut compressed_file = GzipEncoder::new(file);

        if let Some(res) = match_result.details {
            //serialize and write compressed data
            let file_data = serde_json::to_vec(&res.position_data).unwrap();
            compressed_file.write(&file_data).await.unwrap();
        }
    }
}
